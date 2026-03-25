import { Result } from '@resulted/results';
import { PanelRightOpen } from 'lucide-solid';
import { type Component, createSignal } from 'solid-js';
import { Logger } from '../utilities/Logger';
import {
    type MapTileProvider,
    mapTileProviders,
} from '../utilities/mapTileProviders';
import { ResizeDragger } from './ResizeDragger';
import { Scene } from './Scene';
import { SideArea } from './SideArea';

const SIDE_AREA_HIDE_WIDTH = 128;
const SIDE_AREA_DEFAULT_WIDTH = 256;
const SIDE_AREA_MIN_WIDTH = 192;
const SIDE_AREA_MAX_WIDTH = 640;
const TILE_SIZE = 256;

const logger = Logger.new('Editor');

const loadTileWithURL = async (
    url: string,
): Promise<
    Result<
        Uint8Array,
        | 'tileFetchFailed'
        | 'tileBlobFailed'
        | 'tileImageBitmapFailed'
        | 'tileCanvasContextFailed'
    >
> => {
    const response = await Result.try(fetch(url));
    if (response.isErr()) {
        logger.error('tileFetchFailed', { error: response.error, url });
        return Result.err('tileFetchFailed');
    }
    if (!response.value.ok) {
        logger.error('tileFetchFailed', { status: response.value.status, url });
        return Result.err('tileFetchFailed');
    }

    const blob = await Result.try(response.value.blob());
    if (blob.isErr()) {
        logger.error('tileBlobFailed', { error: blob.error, url });
        return Result.err('tileBlobFailed');
    }

    const imageBitmap = await Result.try(createImageBitmap(blob.value));
    if (imageBitmap.isErr()) {
        logger.error('tileImageBitmapFailed', {
            error: imageBitmap.error,
            url,
        });
        return Result.err('tileImageBitmapFailed');
    }

    const canvas = new OffscreenCanvas(TILE_SIZE, TILE_SIZE);
    const ctx = canvas.getContext('2d');
    if (!ctx) {
        logger.error('tileCanvasContextFailed', { url });
        return Result.err('tileCanvasContextFailed');
    }
    ctx.drawImage(imageBitmap.value, 0, 0);

    // Close the image bitmap to release resources ASAP.
    imageBitmap.value.close();

    const imageData = ctx.getImageData(0, 0, TILE_SIZE, TILE_SIZE).data.buffer;

    return Result.ok(new Uint8Array(imageData));
};

export const Editor: Component = () => {
    const [sideAreaWidth, setSideAreaWidth] = createSignal(
        SIDE_AREA_DEFAULT_WIDTH,
    );
    const [mapTileProvider, _setMapTileProvider] =
        createSignal<MapTileProvider>(
            mapTileProviders.providers[mapTileProviders.default],
        );

    const getQuadKey = (x: number, y: number, lod: number) => {
        let quadKey = '';
        for (let i = lod; i > 0; i--) {
            let digit = 0;
            const mask = 1 << (i - 1);
            if ((x & mask) !== 0) {
                digit += 1;
            }
            if ((y & mask) !== 0) {
                digit += 2;
            }
            quadKey += digit.toString();
        }
        return quadKey;
    };

    return (
        <div class="w-full h-full flex select-none bg-zinc-800">
            <SideArea
                hidden={sideAreaWidth() < SIDE_AREA_HIDE_WIDTH}
                width={Math.max(
                    SIDE_AREA_MIN_WIDTH,
                    Math.min(SIDE_AREA_MAX_WIDTH, sideAreaWidth()),
                )}
            />
            <ResizeDragger
                accessibilityValue={sideAreaWidth()}
                direction="horizontal"
                hidden={sideAreaWidth() < SIDE_AREA_HIDE_WIDTH}
                onDoubleClick={() => {
                    setSideAreaWidth(SIDE_AREA_DEFAULT_WIDTH);
                }}
                onResize={(delta) => {
                    setSideAreaWidth((w) => w + delta);
                }}
            />
            <div class="w-full h-full relative">
                {sideAreaWidth() < SIDE_AREA_HIDE_WIDTH ? (
                    <div class="absolute top-2 left-2 text-xs bg-zinc-800 border border-zinc-600/20 rounded-xl p-1 shadow-lg">
                        <button
                            class="p-2 rounded-lg transition cursor-pointer hover:bg-zinc-600"
                            onClick={() =>
                                setSideAreaWidth(SIDE_AREA_DEFAULT_WIDTH)
                            }
                            type="button"
                        >
                            <PanelRightOpen class="w-4 h-4" />
                        </button>
                    </div>
                ) : null}
                <div class="absolute bottom-2 right-2 rounded-full bg-zinc-800/50 pointer-events-none text-xs px-2 py-1 backdrop-blur-sm">
                    {mapTileProvider().attribution}
                </div>
                <Scene
                    class="w-full h-full outline-none"
                    onSceneReady={(scene) => {
                        scene.onImageTileRequested = async (
                            data: Uint32Array,
                        ) => {
                            const request: {
                                x: number;
                                y: number;
                                lod: number;
                            }[] = [];
                            for (let i = 0; i < data.length; i += 3) {
                                request.push({
                                    lod: data[i + 2],
                                    x: data[i],
                                    y: data[i + 1],
                                });
                            }

                            const urlTemplate = mapTileProvider().url;
                            await Promise.all(
                                request
                                    .map(({ x, y, lod }) => {
                                        const quadKey = getQuadKey(x, y, lod);
                                        const url = urlTemplate.replace(
                                            '{quadKey}',
                                            quadKey,
                                        );
                                        return {
                                            tileLOD: lod,
                                            tileX: x,
                                            tileY: y,
                                            url,
                                        };
                                    })
                                    .map(async (tileRequest) => {
                                        const tileData = await loadTileWithURL(
                                            tileRequest.url,
                                        );
                                        if (tileData.isErr()) return;
                                        const packed = new Uint32Array([
                                            tileRequest.tileX,
                                            tileRequest.tileY,
                                            tileRequest.tileLOD,
                                        ]);
                                        scene.sendImageTileLoaded(
                                            packed,
                                            tileData.value,
                                        );
                                    }),
                            );
                        };
                    }}
                />
            </div>
        </div>
    );
};

import { PanelRightOpen } from 'lucide-solid';
import { type Component, createSignal } from 'solid-js';
import { useI18n } from '../utilities/hooks/i18n';
import { ResizeDragger } from './ResizeDragger';
import { Scene } from './Scene';
import { SideArea } from './SideArea';

const SIDE_AREA_HIDE_WIDTH = 128;
const SIDE_AREA_DEFAULT_WIDTH = 256;
const SIDE_AREA_MIN_WIDTH = 192;
const SIDE_AREA_MAX_WIDTH = 640;

export const Editor: Component = () => {
    const i18n = useI18n();
    const [sideAreaWidth, setSideAreaWidth] = createSignal(
        SIDE_AREA_DEFAULT_WIDTH,
    );

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
                                setSideAreaWidth(SIDE_AREA_MIN_WIDTH)
                            }
                            title={i18n.t(
                                'editor.sidepanel.showPanelButtonTooltip',
                            )}
                            type="button"
                        >
                            <PanelRightOpen class="w-4 h-4" />
                        </button>
                    </div>
                ) : null}
                <Scene
                    class="w-full h-full outline-none"
                    onSceneReady={(scene) => {
                        console.log(scene, 2);
                    }}
                />
            </div>
        </div>
    );
};

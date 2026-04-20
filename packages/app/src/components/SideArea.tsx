import type { Component } from 'solid-js';
import { useI18n } from '../utilities/hooks/i18n';
import { mapTileProviders } from '../utilities/mapTileProviders';

export interface SideAreaProps {
    width: number;
    hidden: boolean;
    mapTileProvider: string;
    onMapTileProviderChange: (provider: string) => void;
}

export const SideArea: Component<SideAreaProps> = (props: SideAreaProps) => {
    const i18n = useI18n();

    return (
        <div
            class={`shrink-0 h-full flex-col gap-1 p-2 bg-zinc-800 ${props.hidden ? 'hidden' : 'flex'}`}
            style={{ width: `${props.width}px` }}
        >
            <h1 class="text-lg font-bold text-zinc-300">
                {i18n.t('app.name')}
            </h1>
            <div class="flex items-center justify-between">
                <p class="text-sm text-zinc-500">
                    {i18n.t('editor.sidepanel.tileProvider')}
                </p>
                <select
                    aria-label={i18n.t('editor.sidepanel.tileProvider')}
                    class="bg-zinc-700 text-zinc-300 text-sm rounded-md p-1"
                    onChange={(e) => {
                        const selectedKey = e.currentTarget
                            .value as keyof typeof mapTileProviders.providers;
                        if (!(selectedKey in mapTileProviders.providers))
                            return;
                        props.onMapTileProviderChange(selectedKey);
                    }}
                    value={props.mapTileProvider}
                >
                    {Object.entries(mapTileProviders.providers).map(
                        ([key, provider]) => (
                            <option value={key}>{provider.name}</option>
                        ),
                    )}
                </select>
            </div>
        </div>
    );
};

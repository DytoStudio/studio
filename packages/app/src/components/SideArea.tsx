import type { Component } from 'solid-js';
import { useI18n } from '../utilities/hooks/i18n';

export interface SideAreaProps {
    width: number;
    hidden: boolean;
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
            <p class="text-sm text-zinc-500">
                {i18n.t('editor.sidepanel.placeholder')}
            </p>
        </div>
    );
};

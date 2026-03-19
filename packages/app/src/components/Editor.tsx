import { type Component, createUniqueId, onMount } from 'solid-js';
import { useI18n } from '../utilities/hooks/i18n';

export const Editor: Component = () => {
    const i18n = useI18n();
    const canvasId = createUniqueId();
    let canvas: HTMLCanvasElement | undefined;

    onMount(async () => {
        const dytoScene = await import('@dytostudio/scene');
        await dytoScene.default();
        const scene = new dytoScene.DytoScene(`#${canvasId}`);
        scene.run();
    });

    return (
        <div class="h-full w-full flex flex-col items-center justify-center gap-4">
            <h1>{i18n.t('helloWorld')}</h1>
            <button
                onClick={() =>
                    i18n.setLocale(
                        i18n.locale() === 'en-US' ? 'zh-CN' : 'en-US',
                    )
                }
                type="button"
            >
                change language
            </button>
            <canvas class="h-full w-full" id={canvasId} ref={canvas} />
        </div>
    );
};

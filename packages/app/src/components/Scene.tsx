import type { DytoScene } from '@dytostudio/scene';
import { type Component, createUniqueId, onCleanup, onMount } from 'solid-js';

export const Scene: Component<{
    onSceneReady?: (scene: DytoScene) => void;
    class?: string;
}> = (props) => {
    let scene: DytoScene | null = null;
    const canvasId = createUniqueId();

    onMount(async () => {
        const dyto = await import('@dytostudio/scene');
        await dyto.default();
        const dytoScene = new dyto.DytoScene(`#${canvasId}`);
        dytoScene.run();
        scene = dytoScene;

        props.onSceneReady?.(scene);
    });

    onCleanup(() => {
        scene?.free();
    });

    return <canvas class={props.class} id={canvasId} />;
};

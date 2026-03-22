import type { Component } from 'solid-js';
import { SupportedLocales, useI18n } from '../utilities/hooks/i18n';
import { Scene } from './Scene';

export const Editor: Component = () => {
    const i18n = useI18n();

    return (
        <div class="h-full w-full flex flex-col overflow-hidden">
            <div class="w-full shrink-0 h-12 bg-zinc-900 border-b border-zinc-700 flex items-center px-4 gap-4">
                <h1 class="text-lg font-bold text-zinc-100">
                    {i18n.t('app.name')}
                </h1>
                <button
                    onClick={() => {
                        const currentLocale = i18n.locale();
                        const currentIndex = SupportedLocales.indexOf(currentLocale);
                        const newIndex =
                            (currentIndex + 1) % SupportedLocales.length;
                        const newLocale = SupportedLocales[newIndex];
                            
                        i18n.setLocale(newLocale);
                    }}
                    type="button"
                >
                    change language
                </button>
            </div>
            <div class="flex grow min-h-0 overflow-hidden">
                <div class="shrink-0 w-md h-full bg-zinc-900 border-r border-zinc-700">
                    <div class="p-4 text-sm text-zinc-500">
                        {i18n.t('editor.sidepanel.placeholder')}
                    </div>
                </div>
                <Scene
                    class="w-full! h-full! min-w-0! min-h-0!"
                    onSceneReady={(scene) => {
                        console.log(scene, 2);
                    }}
                />
            </div>
        </div>
    );
};

import type { Component } from 'solid-js';
import { Editor } from './components/Editor';
import { I18nProvider } from './components/providers/I18nProvider';

export const App: Component = () => {
    return (
        <I18nProvider value={null}>
            <Editor />
        </I18nProvider>
    );
};

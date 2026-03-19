import type { ParentComponent } from 'solid-js';
import {
    createI18n,
    I18nContext,
    type I18nContextValue,
    type Locale,
    SupportedLocales,
} from '../../utilities/hooks/i18n';

export const I18nProvider: ParentComponent<{
    value: I18nContextValue | null;
}> = (props) => {
    // Detect the preferred language
    const preferredLanguage = navigator.languages.find((lang) =>
        SupportedLocales.includes(lang as Locale),
    ) as Locale | undefined;

    const i18nValue = props.value ?? createI18n(preferredLanguage ?? 'en-US');
    return (
        <I18nContext.Provider value={i18nValue}>
            {props.children}
        </I18nContext.Provider>
    );
};

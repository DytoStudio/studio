import { Result } from '@resulted/results';
import {
    type Flatten,
    flatten,
    type NullableTranslator,
    resolveTemplate,
    translator,
} from '@solid-primitives/i18n';
import {
    type Accessor,
    createContext,
    createResource,
    createSignal,
    useContext,
} from 'solid-js';
import type en from '../../i18n/en-US.json';
import { Logger } from '../Logger.ts';

/**
 * Module logger.
 */
const logger = Logger.new('i18n');

/**
 * Supported locales in the application.
 */
export const SupportedLocales = ['en-US', 'zh-CN'] as const;

/**
 * Supported locales in the application.
 */
export type Locale = (typeof SupportedLocales)[number];

/**
 * The raw dictionary type for the default locale.
 */
export type RawDictionary = typeof en;
/**
 * The flattened dictionary type used for translation, derived from the raw
 * dictionary.
 */
export type Dictionary = Flatten<RawDictionary>;

/**
 * Value type for the i18n context.
 */
export interface I18nContextValue {
    locale: Accessor<Locale>;
    setLocale: (l: Locale) => void;
    t: NullableTranslator<Dictionary>;
    dictionary: Accessor<Dictionary | null>;
    ready: Accessor<boolean>;
}

/**
 * Context for internationalization (i18n) in the application.
 */
export const I18nContext = createContext<I18nContextValue>();

/**
 * Load a locale's dictionary asynchronously.
 * @param locale The locale to load the dictionary for.
 * @returns The loaded dictionary or an error if loading fails.
 */
export const loadDictionary = async (locale: Locale): Promise<Dictionary> => {
    const dictionary = (
        await Result.try<RawDictionary>(import(`../../i18n/${locale}.json`))
    ).map((dictionary) => flatten(dictionary));

    if (dictionary.isErr()) {
        logger.error(
            `Failed to load dictionary for locale ${locale}`,
            dictionary.error,
        );
        // safety: the requirements of the library we're using for translation
        //         require us to throw an error if the dictionary fails to load
        throw new Error('Failed to load dictionary');
    }

    return dictionary.value;
};

export const createI18n = (initialLocale: Locale): I18nContextValue => {
    const [locale, setLocale] = createSignal<Locale>(initialLocale);
    const [dictionary] = createResource(locale, loadDictionary);

    const t = translator(dictionary, resolveTemplate);

    return {
        dictionary: () => dictionary() ?? null,
        locale,
        ready: () => dictionary() !== undefined,
        setLocale,
        t,
    };
};

export const useI18n = (): I18nContextValue => {
    const context = useContext(I18nContext);
    if (!context) {
        logger.errorAndThrow('not-in-i18n-provider');
        return undefined as never;
    }

    return context;
};

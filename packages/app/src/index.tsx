/* @refresh reload */

import { Result } from '@resulted/results';
import { render } from 'solid-js/web';
import 'solid-devtools';
import './index.css';
import { App } from './App.tsx';
import { Logger } from './utilities/Logger.ts';

/**
 * Module logger.
 */
const logger = Logger.new('index');

/**
 * The main entry point of the application.
 * @returns A result with an optional error.
 */
const main = async (): Promise<
    Result<void, 'app-root-missing' | 'app-root-not-element'>
> => {
    // Logging setup.
    // TODO: Replace with something like sentry
    if (
        Logger.setLogFunction((level, module, message, details) => {
            const logLevelNames = ['DEBUG', 'INFO', 'WARN', 'ERROR'];
            const logMessage = `[${logLevelNames[level]}] [${module}] ${message}`;
            if (details) {
                console.log(logMessage, details);
            } else {
                console.log(logMessage);
            }
        }).isErr()
    ) {
        console.error('Failed to set log function for Logger');
    }

    if (Logger.setLogLevel(0).isErr()) {
        console.error('Failed to set log level for Logger');
    }

    // Start the app.
    const appRoot = document.getElementById('dyto-app');

    if (!appRoot) return Result.err('app-root-missing');
    if (!(appRoot instanceof HTMLElement))
        return Result.err('app-root-not-element');

    render(() => <App />, appRoot);

    return Result.ok(undefined);
};

main().then((result) => {
    if (result.isErr()) logger.error(result.error);
});

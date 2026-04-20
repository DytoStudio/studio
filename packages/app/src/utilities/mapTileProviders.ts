/**
 * Map providers available in the application.
 *
 * Each provider includes a name, URL template, and attribution information.
 */
export interface MapTileProvider {
    name: string;
    url: string;
    attribution: string;
}

/**
 * The collection of map providers, including a default provider.
 */
export type MapTileProviders = {
    default: string;
    providers: Record<string, MapTileProvider>;
};

export const mapTileProviders = {
    default: '4565564b-20c7-4303-8c9b-e226399d6cea',
    providers: {
        '4565564b-20c7-4303-8c9b-e226399d6cea': {
            attribution: '© Microsoft Corporation',
            name: 'Bing Aerial Maps',
            url: 'https://ecn.t0.tiles.virtualearth.net/tiles/a{quadKey}.jpeg?g=15496&pr=odbl&n=z',
        },
        'ba8ea4a8-8aa8-427c-9426-dfe3c46d38b5': {
            attribution: '© Esri',
            name: 'Esri World Imagery',
            url: 'https://services.arcgisonline.com/arcgis/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}',
        },
    },
} as const satisfies MapTileProviders;

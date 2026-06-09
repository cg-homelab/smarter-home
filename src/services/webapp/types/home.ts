export interface HomeLocation {
    latitude: number
    longitude: number
}

export interface Home {
    id: string
    name: string
    address: string
    location: HomeLocation
    locationHashHigh: string
    locationHashMedium: string
    locationHashLow: string
    writeToken: string
    isFavorite: boolean
}

export interface NewHome {
    name: string
    address: string
    location: HomeLocation
}

export interface UpdateHome {
    name: string
    address: string
    location: HomeLocation
}

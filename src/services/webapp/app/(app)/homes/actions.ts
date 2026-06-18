'use server'

import { apiFetch } from '@/lib/api'
// import { revalidateTag } from 'next/cache'

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

export async function getHomes(): Promise<Home[]> {
    return await apiFetch<Home[]>('/home', { tags: ['homes'] })
}

export async function createHomeAction(data: NewHome): Promise<Home> {
    const home = await apiFetch<Home>('/home', {
        method: 'POST',
        body: JSON.stringify(data),
    })
    //   revalidateTag("homes");
    return home
}

export async function updateHomeAction(
    id: string,
    data: UpdateHome,
): Promise<Home> {
    const home = await apiFetch<Home>(`/home/${id}`, {
        method: 'PUT',
        body: JSON.stringify(data),
    })
    //   revalidateTag("homes");
    return home
}

export async function deleteHomeAction(id: string): Promise<void> {
    await apiFetch<void>(`/home/${id}`, { method: 'DELETE' })
    //   revalidateTag("homes");
}

export async function setFavoriteHomeAction(
    id: string,
    isFavorite: boolean,
): Promise<Home> {
    return await apiFetch<Home>(`/home/${id}/favorite`, {
        method: 'PATCH',
        body: JSON.stringify({ isFavorite }),
    })
}

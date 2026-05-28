'use server'

import { apiFetch } from '@/lib/api'
// import { revalidateTag } from 'next/cache'

export interface Home {
    id: string
    name: string
    address: string
    writeToken: string
}

export interface NewHome {
    name: string
    address: string
}

export interface UpdateHome {
    name: string
    address: string
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

import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs))
}

export function capitalizeFirstLetter(input: string): string {
    if (!input) {
        return input // Return the input as is if it's empty or undefined
    }
    return input.charAt(0).toUpperCase() + input.slice(1)
}

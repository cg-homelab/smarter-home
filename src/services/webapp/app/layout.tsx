import type { Metadata } from 'next'
import { AuthProvider } from '@/contexts/auth-context'
import { Roboto, Roboto_Mono } from 'next/font/google'
import './globals.css'

const roboto = Roboto({
    variable: '--font-sans',
    subsets: ['latin'],
})

const robotoMono = Roboto_Mono({
    variable: '--font-roboto-mono',
    subsets: ['latin'],
})

export const metadata: Metadata = {
    title: 'Smarter Home',
    description: 'Energy monitoring and analytics dashboard',
}

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode
}>) {
    return (
        <html
            suppressHydrationWarning
            lang="en"
            className={`${roboto.variable} ${robotoMono.variable} h-full antialiased`}
        >
            <body className="min-h-full flex flex-col">
                <AuthProvider>
                    <ThemeProvider
                        attribute="class"
                        defaultTheme="system"
                        enableSystem
                    >
                        {children}
                    </ThemeProvider>
                </AuthProvider>
            </body>
        </html>
    )
}

import './globals.css'
import { ThemeProvider } from '@/components/theme/theme-provider'

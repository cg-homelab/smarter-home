import './globals.css';

import { Analytics } from '@vercel/analytics/react';

export const metadata = {
  title: 'Smarter Home',
  description:
    'Data analaytics for your home. Understand your energy usage, and make smarter choices.',
};

export default function RootLayout({
  children
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="flex min-h-screen w-full flex-col">{children}</body>
      <Analytics />
    </html>
  );
}

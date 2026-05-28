import { UserDropdown } from "@/components/navigation/user-dropdown";
import * as React from "react";
import { BreadcrumbNavigation } from "@/components/navigation/breadcrumb-navigation";
import { ModeToggle } from "@/components/theme/mode-toggle";
import { TopNavigation } from "@/components/navigation/top-nav";

export default function AppGroupLayout({
  children,
}: {
  children: React.ReactNode;
}) {

  return (
    <div className="min-h-screen bg-background">
      <header className="sticky top-0 z-50 w-full border-b border-border bg-background/80 backdrop-blur-sm supports-[backdrop-filter]:bg-background/60">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between gap-4">
          <div className="flex items-center gap-6">
            <h1 className="text-xl font-bold">Smarter Home</h1>
            <TopNavigation />
          </div>
          <div className="flex items-center gap-2">
            <ModeToggle />
            <UserDropdown />
          </div>
        </div>
      </header>
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-4">
        <BreadcrumbNavigation />
      </div>
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        {children}
      </main>
    </div>
  );
}

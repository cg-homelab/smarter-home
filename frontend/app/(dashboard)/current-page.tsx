'use client';

import { usePathname } from 'next/navigation';
import Link from 'next/link';
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator
} from '@/components/ui/breadcrumb';

export function CurrentPage(){
  const fullpath = usePathname();
  const pathArray = fullpath.split('/').filter(Boolean);
  const pathParts: string[] = [];

  for (let i = 0; i < pathArray.length; i++) {
      pathParts.push('/');
      pathParts.push(pathArray[i]);
  }

  const breadcrumbs = pathParts.map((path, index) => {
    if (path === '/') {
      return <BreadcrumbSeparator key={index}/>;
    }
    return <PageLevel path={path} pathname={path} />;
  });
  
  return (
    <Breadcrumb className="hidden md:flex">
      <BreadcrumbList>
        <BreadcrumbItem key="home">
          <BreadcrumbLink asChild>
            <Link href="/">Home</Link>
          </BreadcrumbLink>
        </BreadcrumbItem>
        {breadcrumbs}
      </BreadcrumbList>
    </Breadcrumb>
  );
}
function PageLevel ({path, pathname}: {path: string, pathname: string}) {
  // Convert first letter to uppercase and remove the first character
  const name = pathname.charAt(0).toUpperCase() + pathname.slice(1);
  return (
    <BreadcrumbItem key={name}>
      <BreadcrumbLink asChild>
        <Link href={path}>{name}</Link>
      </BreadcrumbLink>
    </BreadcrumbItem>
  );
}

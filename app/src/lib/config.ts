export type Route = {
  title: string;
  href: string;
  label: string;
  icon: string;
  variant: "default" | "ghost";
};

import { base } from "$app/paths";

export const routes: Route[] = [
  {
    title: "Wad Walker",
    href: base + "/wad",
    label: "",
    icon: "mdi:files",
    variant: "default",
  },
  {
    title: "Bin Peek",
    href: base + "/bin",
    label: "",
    icon: "carbon:tree-view",
    variant: "default",
  },

] as const;

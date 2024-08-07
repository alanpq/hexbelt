export type Route = {
  title: string;
  href: string;
  label: string;
  icon: string;
  variant: "default" | "ghost";
};

export const routes: Route[] = [
  {
    title: "Wad Walker",
    href: "/wad",
    label: "",
    icon: "mdi:file",
    variant: "default",
  },
  {
    title: "Something else",
    href: "#",
    label: "",
    icon: "",
    variant: "default",
  },

] as const;
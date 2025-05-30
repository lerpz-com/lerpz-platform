import { BookOpen, Bot, Settings2, SquareTerminal } from "lucide-react";

const nav = [
  {
    title: "Overview",
    url: "",
    content: [
      {
        title: "Violations",
        url: "",
        icon: SquareTerminal,
        isActive: true,
        items: [
          {
            title: "History",
            url: "/history",
          },
          {
            title: "Settings",
            url: "/settings",
          },
        ],
      },
    ],
  },
  {
    title: "Admin",
    url: "/admin",
    content: [
      {
        title: "Users",
        url: "/users",
        icon: Bot,
        isActive: true,
        items: [
          {
            title: "Manage Users",
            url: "/manage",
          },
        ],
      },
      {
        title: "Config",
        url: "/config/users",
        icon: Settings2,
        isActive: true,
        items: [
          {
            title: "Channels",
            url: "/channels",
          },
          {
            title: "Rules",
            url: "/rules",
          },
        ],
      },
    ],
  },
];

export default nav;

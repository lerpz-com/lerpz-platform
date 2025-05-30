import { Bot, Settings2, SquareTerminal } from "lucide-react"

const nav = [
  {
    title: "Overview",
    url: "",
    content: [
      {
        title: "Violations",
        url: "/violations",
        icon: SquareTerminal,
        isActive: true,
        items: [
          {
            title: "My Violations",
            url: "/my-violations"
          },
          {
            title: "All Violations",
            url: "/all-violations"
          }
        ]
      }
    ]
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
            url: "/manage"
          },
          {
            title: "Access Control",
            url: "/access-control"
          }
        ]
      },
      {
        title: "Config",
        url: "/config",
        icon: Settings2,
        isActive: true,
        items: [
          {
            title: "Connected Apps",
            url: "/connected-app"
          },
          {
            title: "Rule Management",
            url: "/rules"
          },
          {
            title: "Channel management",
            url: "/channels"
          }
        ]
      }
    ]
  }
]

export default nav

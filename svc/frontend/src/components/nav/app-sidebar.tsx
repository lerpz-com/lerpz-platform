"use client"

import {
  AudioWaveform,
  BookOpen,
  Command,
  Frame,
  GalleryVerticalEnd,
  Gauge,
  PieChart,
  Settings2,
  SquarePen,
  User
} from "lucide-react"

import { NavGroup } from "@/components/nav/nav-group"
import { NavOther } from "@/components/nav/nav-other"
import { NavUser } from "@/components/nav/nav-user"
import { TeamSwitcher } from "@/components/nav/team-switcher"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarRail
} from "@/components/ui/sidebar"

const user = {
  name: "Kanerix",
  email: "kas@lerpz.com",
  avatar: "/avatars/shadcn.jpg"
}

const teams = [
  {
    name: "Lerpz Com.",
    logo: GalleryVerticalEnd,
    plan: "Enterprise"
  },
  {
    name: "Acme Inc.",
    logo: AudioWaveform,
    plan: "Startup"
  },
  {
    name: "Evil Corp.",
    logo: Command,
    plan: "Free"
  }
]

const navGeneral = [
  {
    title: "Overview",
    url: "overview",
    icon: Gauge,
    isActive: false,
    items: [
      {
        title: "Violations",
        url: "violations"
      }
    ]
  },
  {
    title: "Documentation",
    url: "documentation",
    icon: BookOpen,
    items: [
      {
        title: "Introduction",
        url: "introduction"
      },
      {
        title: "Get Started",
        url: "get-started"
      },
      {
        title: "Tutorials",
        url: "tutorials"
      },
      {
        title: "Changelog",
        url: "changelog"
      }
    ]
  }
]

const navAdmin = [
  {
    title: "Users",
    url: "#",
    icon: User,
    isActive: false,
    items: [
      {
        title: "Invitations",
        url: "#"
      }
    ]
  },
  {
    title: "Configuration",
    url: "configuration",
    icon: SquarePen,
    isActive: false,
    items: [
      {
        title: "Rules",
        url: "rules"
      },
      {
        title: "Channels",
        url: "channels"
      }
    ]
  },
  {
    title: "Settings",
    url: "settings",
    icon: Settings2,
    items: [
      {
        title: "General",
        url: "general"
      },
      {
        title: "Team",
        url: "team"
      },
      {
        title: "Billing",
        url: "billing"
      },
      {
        title: "Limits",
        url: "limits"
      }
    ]
  }
]

const projects = [
  {
    name: "Design Engineering",
    url: "design-engineering",
    icon: Frame
  },
  {
    name: "Sales & Marketing",
    url: "sales-marketing",
    icon: PieChart
  }
]

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <TeamSwitcher teams={teams} />
      </SidebarHeader>
      <SidebarContent>
        <NavGroup title="General" items={navGeneral} />
        <NavGroup title="Admin" items={navAdmin} />
        <NavOther projects={projects} />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={user} />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}

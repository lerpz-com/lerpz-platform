"use client"

import { Moon, Sun } from "lucide-react"
import { useTheme } from "next-themes"
import { Button } from "@/components/ui/button"

export function ThemeButton() {
  const { setTheme } = useTheme()

  const rotateTheme = () => {
    setTheme((currentTheme) => {
      if (currentTheme === "dark") {
        return "light"
      } else {
        return "dark"
      }
    })
  }

  return (
    <Button variant="ghost" size="icon" onClick={rotateTheme}>
      <Sun className="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all dark:scale-0 dark:-rotate-90" />
      <Moon className="h-[1.2rem] w-[1.2rem] scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0 absolute" />
      <span className="sr-only">Toggle theme</span>
    </Button>
  )
}

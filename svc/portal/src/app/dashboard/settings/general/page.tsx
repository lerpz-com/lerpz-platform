"use client"

import { useState } from "react"

import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from "@/components/ui/card"
import { Separator } from "@/components/ui/separator"

export default function IntegrationsPage() {
  const [connected, setConnected] = useState(false)
  const handleConnect = async () => {
    setConnected((v) => !v)
  }

  return (
    <div className="flex flex-col gap-4">
      <h1 className="text-2xl font-semibold">Integrations</h1>
      <p className="text-muted-foreground">
        Connect your platforms to automate scanning.
      </p>
      <Card className="w-full max-w-sm">
        <CardHeader>
          <CardTitle className="flex items-start justify-between">
            Entra ID
          </CardTitle>
          <CardDescription>
            Connect your Entra ID tenant to automate scanning of your data.
          </CardDescription>
        </CardHeader>
        <Separator />
        <CardContent>
          {connected ? (
            <CardDescription>
              Your currently have connected an Entra ID tenant.
            </CardDescription>
          ) : (
            <CardDescription>
              Your currently have not connected an Entra ID tenant.
            </CardDescription>
          )}
        </CardContent>
        <CardFooter className="flex-col gap-2">
          {connected ? (
            <Button
              type="submit"
              className="w-full"
              variant="destructive"
              onClick={handleConnect}
            >
              Disconnect
            </Button>
          ) : (
            <Button type="submit" className="w-full" onClick={handleConnect}>
              Connect
            </Button>
          )}
        </CardFooter>
      </Card>
    </div>
  )
}

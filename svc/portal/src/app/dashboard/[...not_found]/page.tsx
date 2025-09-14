import Link from "next/link"

export default function PageNotFound() {
  return (
    <div className="flex flex-col items-center justify-center h-full">
      <h1 className="font-bold">Not Found – 404!</h1>
      <p>The page you&apos;re looking for doesn&apos;t exist.</p>
      <Link href="/dashboard/overview/violations">Go back to Overview</Link>
    </div>
  )
}

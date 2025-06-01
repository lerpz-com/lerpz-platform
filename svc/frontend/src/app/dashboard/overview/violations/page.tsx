"use client"

import { DataTable } from "@/components/tables/violations-table"

const data = [
  {
    id: 1,
    header: "Violation 1",
    type: "Type A",
    status: "Open",
    detected: "2023-10-01",
    reviewer: "John Doe"
  },
  {
    id: 2,
    header: "Violation 2",
    type: "Type B",
    status: "Closed",
    detected: "2023-10-02",
    reviewer: "Jane Smith"
  },
  {
    id: 3,
    header: "Violation 3",
    type: "Type A",
    status: "In Progress",
    detected: "2023-10-03",
    reviewer: "Mike Johnson"
  },
  {
    id: 4,
    header: "Violation 4",
    type: "Type C",
    status: "Open",
    detected: "2023-10-04",
    reviewer: "Sarah Wilson"
  },
  {
    id: 5,
    header: "Violation 5",
    type: "Type B",
    status: "Closed",
    detected: "2023-10-05",
    reviewer: "David Brown"
  },
  {
    id: 6,
    header: "Violation 6",
    type: "Type A",
    status: "Open",
    detected: "2023-10-06",
    reviewer: "Lisa Davis"
  },
  {
    id: 7,
    header: "Violation 7",
    type: "Type C",
    status: "In Progress",
    detected: "2023-10-07",
    reviewer: "Tom Anderson"
  },
  {
    id: 8,
    header: "Violation 8",
    type: "Type B",
    status: "Open",
    detected: "2023-10-08",
    reviewer: "Emily Taylor"
  },
  {
    id: 9,
    header: "Violation 9",
    type: "Type A",
    status: "Closed",
    detected: "2023-10-09",
    reviewer: "Chris Martinez"
  },
  {
    id: 10,
    header: "Violation 10",
    type: "Type C",
    status: "Open",
    detected: "2023-10-10",
    reviewer: "Amanda White"
  },
  {
    id: 11,
    header: "Violation 11",
    type: "Type B",
    status: "In Progress",
    detected: "2023-10-11",
    reviewer: "Robert Clark"
  },
  {
    id: 12,
    header: "Violation 12",
    type: "Type A",
    status: "Open",
    detected: "2023-10-12",
    reviewer: "Michelle Lee"
  },
  {
    id: 13,
    header: "Violation 13",
    type: "Type C",
    status: "Closed",
    detected: "2023-10-13",
    reviewer: "Kevin Rodriguez"
  },
  {
    id: 14,
    header: "Violation 14",
    type: "Type B",
    status: "Open",
    detected: "2023-10-14",
    reviewer: "Nicole Garcia"
  },
  {
    id: 15,
    header: "Violation 15",
    type: "Type A",
    status: "In Progress",
    detected: "2023-10-15",
    reviewer: "James Thompson"
  }
]

export default function AllViolationsPage() {
  return (
    <div className="flex flex-col items-center h-full">
      <DataTable data={data} />
    </div>
  )
}

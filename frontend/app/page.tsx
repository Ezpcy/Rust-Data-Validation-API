import TableProvider from "@/providers/TableProvider";

export const revalidate = 0;

export default function Home() {

  return (
    <main className="space-y-3">
      <h1 className="text-3xl font-bold">
        User Validation with Rust and MongoDB
      </h1>
      <TableProvider />
    </main>
  );
}

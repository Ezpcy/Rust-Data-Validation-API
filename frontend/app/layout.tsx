import ModalProvider from "@/providers/ModalProvider";
import "./globals.css";
import type { Metadata } from "next";
import { Figtree } from "next/font/google";
import ToasterProvider from "@/providers/ToasterProvider";

const font = Figtree({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "User Validation with Rust and MongoDB",
  description: "User Validation with Rust and MongoDB",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={font.className}>
        <ToasterProvider />
        <div className="m-5">
          <ModalProvider  />
          {children}
        </div>
      </body>
    </html>
  );
}

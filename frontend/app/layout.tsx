import "@/public/styles/globals.css";
import { Inter } from "next/font/google";
import { AuthProvider } from "@/context/AuthContext";

const inter = Inter({ subsets: ["latin"] });

export const metadata = {
  title: "Ataria Game Engine",
  description:
    "Ataria is a game engine aimed for the versatiles who yearns to create, share, and play your own games",
  openGraph: {
    title: "Ataria Game Engine",
    description:
      "Ataria is a game engine aimed for the versatiles who yearns to create, share, and play your own games",
    url: "https://your-site-url.com",
    type: "website",
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <AuthProvider>{children}</AuthProvider>
      </body>
    </html>
  );
}

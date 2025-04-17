import Link from "next/link";
import React from "react";
import { cn } from "@/lib/utils";

interface ThreeDBtnProps {
    href: string;
    text: string;
    variant?: "black" | "white" | "purple" | "blue" | "red";
    className?: string;
}

const ThreeDBtn = ({
    href,
    text,
    variant = "purple",
    className = "",
}: ThreeDBtnProps) => {
    const gradientClasses = {
        black: "bg-gradient-to-br from-black via-gray-800 to-white/10",
        white: "bg-gradient-to-br from-white via-gray-200 to-black/10",
        purple: "bg-gradient-to-br from-[#4C43CD] via-purple-500 to-white/10",
        blue: "bg-gradient-to-br from-blue-600 via-indigo-500 to-white/20",
        red: "bg-gradient-to-br from-red-600 via-pink-500 to-white/10",
    };

    // Base classes for all variants
    const baseClasses = `
    px-6 py-3 rounded-full shadow-xl h-full relative group
    inset-shadow-sm/50 inset-shadow-white/50 
    cursor-pointer border border-2 border-white/10 
    transition-all duration-300 ease-in-out
    hover:shadow-[0_0_15px_rgba(255,255,255,0.5)] hover:scale-105
  `;

    const frameClasses = cn(gradientClasses[variant], baseClasses, className); // Using cn for clean merging

    const handleClick = () => {
        window.open(href, '_blank', 'noopener,noreferrer');
    };

    return (
        <div className={frameClasses} onClick={handleClick}>
            <Link target="_blank" href={href}>{text}</Link>
        </div>
    );
}

export default ThreeDBtn

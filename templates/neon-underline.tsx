"use client"
import React, { useState, useEffect } from 'react';

interface NeonColor {
    middle: string;
    side: string;
}

interface NeonUnderlineProps {
    children: React.ReactNode;
    colors?: NeonColor[];
    width?: string | number;
    className?: string;
    style?: React.CSSProperties;
    alwaysOn?: boolean;
    cycleSpeed?: number;
}

const NeonUnderline = ({
    children = "Come hover here",
    colors = [
        { middle: '#0ea5e9', side: '#6366f1' }, // Blue/Indigo
        { middle: '#f43f5e', side: '#ec4899' }, // Red/Pink  
        { middle: '#10b981', side: '#059669' }, // Green/Emerald
        { middle: '#f59e0b', side: '#d97706' }, // Amber/Yellow
        { middle: '#8b5cf6', side: '#7c3aed' }  // Purple/Violet
    ],
    width = '75%',
    className = '',
    style = {},
    alwaysOn = false,
    cycleSpeed = 2000 // Time in ms for color cycling when alwaysOn is true
}: NeonUnderlineProps) => {
    const [isHovered, setIsHovered] = useState(false);
    const [colorIndex, setColorIndex] = useState(0);

    // Get current colors
    const currentColors = colors[colorIndex % colors.length];

    // Cycle colors when alwaysOn is true
    useEffect(() => {
        let intervalId: ReturnType<typeof setInterval>;

        if (alwaysOn) {
            intervalId = setInterval(() => {
                setColorIndex(prev => (prev + 1) % colors.length);
            }, cycleSpeed);
        }

        return () => {
            if (intervalId) clearInterval(intervalId);
        };
    }, [alwaysOn, colors.length, cycleSpeed]);

    // Change color on each hover
    const handleMouseEnter = () => {
        setIsHovered(true);
        setColorIndex(prev => (prev + 1) % colors.length);
    };

    const neonStyles = {
        '--om-neon-underline-middle-color': currentColors.middle,
        '--om-neon-underline-side-color': currentColors.side,
        '--om-neon-underline-width': width,
        ...style
    };

    const showEffect = alwaysOn || isHovered;

    return (
        <div
            className={`relative w-full items-center justify-center ${className}`}
            onMouseEnter={handleMouseEnter}
            onMouseLeave={() => setIsHovered(false)}
        >
            <div className='flex items-center justify-center'>
                {children}
            </div>
            <div
                className="relative w-full h-2 transition-opacity duration-300"
                style={{
                    ...neonStyles,
                    opacity: showEffect ? 1 : 0
                }}
            >
                {/* Glow 1 - Wide blur with side color */}
                <div
                    className="absolute top-0 left-0 right-0 mx-auto blur-sm"
                    style={{
                        width: 'var(--om-neon-underline-width)',
                        height: '2px',
                        backgroundImage: 'linear-gradient(to right, transparent, var(--om-neon-underline-side-color), transparent)'
                    }}
                />

                {/* Color 1 - Wide thin line with side color */}
                <div
                    className="absolute top-0 left-0 right-0 mx-auto"
                    style={{
                        width: 'var(--om-neon-underline-width)',
                        height: '1px',
                        backgroundImage: 'linear-gradient(to right, transparent, var(--om-neon-underline-side-color), transparent)'
                    }}
                />

                {/* Glow 2 - Narrow strong blur with middle color */}
                <div
                    className="absolute top-0 left-0 right-0 mx-auto blur-sm"
                    style={{
                        width: 'calc(var(--om-neon-underline-width) * 0.33333)',
                        height: '5px',
                        backgroundImage: 'linear-gradient(to right, transparent, var(--om-neon-underline-middle-color), transparent)'
                    }}
                />

                {/* Color 2 - Narrow thin line with middle color */}
                <div
                    className="absolute top-0 left-0 right-0 mx-auto"
                    style={{
                        width: 'calc(var(--om-neon-underline-width) * 0.33333)',
                        height: '1px',
                        backgroundImage: 'linear-gradient(to right, transparent, var(--om-neon-underline-middle-color), transparent)'
                    }}
                />
            </div>
        </div>
    );
};

export default NeonUnderline;


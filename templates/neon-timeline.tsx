"use client"

import React, { useCallback, useEffect, useRef } from 'react';
import { twMerge } from 'tailwind-merge';
// import ImageCarousel from './ImageCarousel';
import { timelineData } from "./utils/timeline-data"
import "./timeline.css"

// Utility to sanitize HTML content for safe rendering
const sanitizeHtml = (html: string) => {
    return { __html: html };
};

interface ImageItem {
    id: number;
    src: string;
    alt: string;
}

interface TimelineEntry {
    title: string;
    content: string;
    date?: string;
    image?: ImageItem[];
    summaryPoints?: string[];
}

interface TimelineProps {
    data: TimelineEntry[];
    styleClass?: string;
    entriesGap?: string;
    entryGap?: string;
    titleGap?: string;
    pathWidth?: string;
    titleMaxWidth?: string;
    pathColor?: string;
    gradientColors?: [string, string];
}

const Timeline: React.FC<TimelineProps> = ({
    data = timelineData,
    styleClass,
    entriesGap,
    entryGap,
    titleGap,
    pathWidth,
    titleMaxWidth,
    pathColor = '#e2e8f0',
    gradientColors = ['#3b82f6', '#7f00ff'],
}) => {
    const wrapperRef = useRef<HTMLDivElement>(null);
    const entriesRefs = useRef<(HTMLDivElement | null)[]>(Array(data.length).fill(null));
    const timelineSvgRef = useRef<SVGSVGElement>(null);
    const timelinePathRef = useRef<SVGPathElement>(null);
    const timelineGradientPathRef = useRef<SVGPathElement>(null);
    const circleRefs = useRef<(HTMLDivElement | null)[]>([]);

    // Always use window as the scrollable parent for outer page scroll
    const scrollableParent = window;

    // Custom CSS properties for styling
    const customStyles: React.CSSProperties = {
        '--om-timeline-entries-gap': entriesGap || '4rem',
        '--om-timeline-entry-gap': entryGap || '2rem',
        '--om-timeline-entry-title-gap': titleGap || '2rem',
        '--om-timeline-path-width': pathWidth || '2px',
        '--om-timeline-entry-title-max-width': titleMaxWidth || '0rem',
        '--om-timeline-path-color': pathColor,
        '--om-timeline-gradient-start': gradientColors[0],
        '--om-timeline-gradient-end': gradientColors[1],
    } as React.CSSProperties;

    // Utility to debounce a function for performance optimization
    const debounce = (func: (...args: any[]) => void, wait: number) => {
        let timeout: NodeJS.Timeout;
        return (...args: any[]) => {
            clearTimeout(timeout);
            timeout = setTimeout(() => func(...args), wait);
        };
    };

    // Calculate SVG path and circle positions
    const calculateSvgPathAndCircles = (wrapperRect: DOMRect): { path: string; circlePositions: { x: number; y: number }[] } => {
        const entries = entriesRefs.current.filter(Boolean) as HTMLDivElement[];
        if (entries.length === 0) return { path: '', circlePositions: [] };

        let path = '';
        const circlePositions: { x: number; y: number }[] = [];
        const isMobile = window.innerWidth <= 500;
        const curveExtension = isMobile ? 200 : 400; // Controls path curvature
        const padding = 80; // Padding for path positioning

        entries.forEach((entry, i) => {
            const entryRect = entry.getBoundingClientRect();
            const startX = i % 2 === 0 ? padding : wrapperRect.width - padding;
            const endX = i % 2 === 0 ? wrapperRect.width - padding : padding;
            const startY = entryRect.top - wrapperRect.top + 20;
            const endY = entryRect.bottom - wrapperRect.top;

            circlePositions.push({ x: startX, y: startY });

            if (i === 0) {
                path += `M${startX},${startY - 30}`; // Start path at first circle
            }

            const extendedY = endY + 10;
            path += ` L${startX},${extendedY}`; // Draw line to end of entry

            if (i < entries.length - 1) {
                const nextEntryRect = entries[i + 1].getBoundingClientRect();
                const nextStartY = nextEntryRect.top - wrapperRect.top + 60;
                const controlPointY1 = extendedY + curveExtension;
                const controlPointY2 = nextStartY - curveExtension;
                path += ` C${startX},${controlPointY1} ${endX},${controlPointY2} ${endX},${nextStartY}`; // Cubic Bezier curve
            } else {
                path += ` L${startX},${endY + 160}`; // Extend path to end
            }
        });

        return { path, circlePositions };
    };

    // Update SVG path and circle positions
    const updateSvgPathAndCircles = useCallback((wrapperRect: DOMRect) => {
        if (!timelineSvgRef.current || !timelinePathRef.current || !timelineGradientPathRef.current) return;

        const { path, circlePositions } = calculateSvgPathAndCircles(wrapperRect);
        timelineSvgRef.current.setAttribute('width', `${wrapperRect.width}`);
        timelineSvgRef.current.setAttribute('height', `${wrapperRect.height}`);
        timelinePathRef.current.setAttribute('d', path);
        timelineGradientPathRef.current.setAttribute('d', path);

        // Position circles along the path
        circleRefs.current.forEach((circle, index) => {
            if (circle && circlePositions[index]) {
                const { x, y } = circlePositions[index];
                circle.style.left = `${x - 20}px`;
                circle.style.top = `${y - 80}px`;
            }
        });
    }, []);

    // Update the animated gradient path based on window scroll
    const updateTimelineLine = useCallback(() => {
        if (!wrapperRef.current || !timelineGradientPathRef.current) return;

        const rect = wrapperRef.current.getBoundingClientRect();
        const scrollY = window.scrollY; // Current scroll position of the window
        const windowHeight = window.innerHeight; // Viewport height
        // const documentHeight = document.documentElement.scrollHeight; // Total document height

        // Calculate the timeline's position relative to the document
        const timelineTop = rect.top + scrollY; // Absolute top position of timeline
        const timelineHeight = rect.height; // Height of the timeline

        // Calculate scroll progress based on when the timeline is in view
        const scrollProgress = (scrollY + windowHeight - timelineTop) / (timelineHeight + windowHeight);
        const progress = Math.max(0, Math.min(1, scrollProgress)); // Clamp progress between 0 and 1

        // Animate the gradient path
        const length = timelineGradientPathRef.current.getTotalLength();
        timelineGradientPathRef.current.style.strokeDasharray = `${length}`;
        timelineGradientPathRef.current.style.strokeDashoffset = `${length * (1 - progress)}`;
    }, []);

    // Debounced version of updateTimelineLine for scroll events
    const debouncedUpdateTimelineLine = useCallback(debounce(updateTimelineLine, 10), [updateTimelineLine]);

    // Set timeline height and update SVG
    const setHeight = useCallback(() => {
        if (!wrapperRef.current) return;

        const rect = wrapperRef.current.getBoundingClientRect();
        updateSvgPathAndCircles(rect);
        updateTimelineLine();
    }, [updateTimelineLine, updateSvgPathAndCircles]);

    // Effect to handle scroll and resize events
    useEffect(() => {
        setHeight();

        const handleScroll = () => debouncedUpdateTimelineLine();
        const handleResize = () => setHeight();

        window.addEventListener('scroll', handleScroll);
        window.addEventListener('resize', handleResize);

        // Cleanup event listeners
        return () => {
            window.removeEventListener('scroll', handleScroll);
            window.removeEventListener('resize', handleResize);
        };
    }, [setHeight, debouncedUpdateTimelineLine]);

    return (
        <div
            ref={wrapperRef}
            className={twMerge('om-timeline w-full relative switch', styleClass)}
            style={customStyles}
        >
            {/* Timeline entries */}
            <div className="om-timeline-entries relative flex flex-col gap-[--om-timeline-entries-gap] max-w-full">
                {data.map((item, index) => (
                    <div
                        key={index}
                        ref={(el) => {
                            if (el !== null) {
                                entriesRefs.current[index] = el;
                            }
                        }}
                        className={twMerge(
                            'om-timeline-entry flex items-end pt-5 gap-[--om-timeline-entry-gap]',
                            index % 2 !== 0 ? 'right-side flex-row-reverse justify-end' : 'left-side justify-start'
                        )}
                    >
                        {/* Entry header with sticky circle and title */}
                        <div
                            className={twMerge(
                                'om-timeline-entry-header sticky top-0 z-40 flex items-center gap-[--om-timeline-entry-title-gap] relative',
                                index % 2 !== 0 ? 'flex-row-reverse' : 'flex-row mr-auto md:mr-0'
                            )}
                        >
                            <div
                                ref={(el) => {
                                    if (el !== null) {
                                        circleRefs.current[index] = el;
                                    }
                                }}
                                className="om-timeline-circle h-10 w-10 rounded-full bg-black flex items-center justify-center flex-shrink-0"
                                style={{ position: 'sticky', top: '2rem', right: '60px' }}
                            >
                                <div className="om-timeline-inner-circle h-4 w-4 rounded-full bg-gray-800 border border-gray-600"></div>
                            </div>

                            <div className={index % 2 !== 0 ? 'mr-12 text-right' : 'ml-12 text-left'}>
                                <div
                                    className="om-timeline-entry-title text-lg text-black dark:text-white font-semibold"
                                    dangerouslySetInnerHTML={sanitizeHtml(item.title)}
                                />
                                {item.date && <div className="text-sm text-gray-500">{item.date}</div>}
                            </div>
                        </div>

                        {/* Entry content */}
                        <div
                            className={twMerge(
                                'om-timeline-entry-content flex-1 max-w-full',
                                index % 2 !== 0 ? "text-right flex flex-col items-end" : ""
                            )}
                        >
                            {/* {item.image && <ImageCarousel images={item.image} />} */}
                            <div className="text-black dark:text-white font-bold text-sm" dangerouslySetInnerHTML={sanitizeHtml(item.content)} />
                            {item.summaryPoints && (
                                <ul className="list-disc mt-2">
                                    {item.summaryPoints.map((point, i) => (
                                        <li key={i} className="text-sm md:text-md text-gray-500 list-none">{point}</li>
                                    ))}
                                </ul>
                            )}
                        </div>
                    </div>
                ))}
            </div>

            {/* SVG for the timeline path */}
            <div className="om-timeline-line-wrapper absolute top-0 w-full h-full pointer-events-none">
                <svg
                    ref={timelineSvgRef}
                    className="om-timeline-svg absolute"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <defs>
                        <linearGradient id="gradient" x1="0%" y1="0%" x2="0%" y2="100%">
                            <stop offset="0%" style={{ stopColor: 'transparent', stopOpacity: 1 }} />
                            <stop offset="10%" style={{ stopColor: gradientColors[1], stopOpacity: 1 }} />
                            <stop offset="90%" style={{ stopColor: gradientColors[0], stopOpacity: 1 }} />
                            <stop offset="100%" style={{ stopColor: 'transparent', stopOpacity: 1 }} />
                        </linearGradient>
                        <linearGradient id="gradientBg" x1="0%" y1="0%" x2="0%" y2="100%">
                            <stop offset="0%" style={{ stopColor: 'transparent', stopOpacity: 1 }} />
                            <stop offset="10%" style={{ stopColor: pathColor, stopOpacity: 1 }} />
                            <stop offset="90%" style={{ stopColor: pathColor, stopOpacity: 1 }} />
                            <stop offset="100%" style={{ stopColor: 'transparent', stopOpacity: 1 }} />
                        </linearGradient>
                    </defs>
                    <path ref={timelinePathRef} className="om-timeline-path" stroke="url(#gradientBg)" />
                    <path ref={timelineGradientPathRef} className="om-timeline-gradient-path" stroke="url(#gradient)" />
                </svg>
            </div>
        </div>
    );
};

export default Timeline;

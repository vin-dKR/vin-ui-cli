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


export const timelineData: TimelineEntry[] = [
    {
        title: 'Freelance Project: Taskify App',
        content: 'Full-Stack Developer',
        date: 'Jan 2024 - Jun 2024',
        image: [
            {
                id: 1,
                src: '/images/taskify/taskify1.jpg',
                alt: 'Taskify app homepage'
            },
            {
                id: 2,
                src: '/images/taskify/taskify2.jpg',
                alt: 'Taskify task creation UI'
            },
            {
                id: 3,
                src: '/images/taskify/taskify3.jpg',
                alt: 'Taskify mobile app view'
            },
        ],
        summaryPoints: [
            '• Developed a feat: ',
            '• Implemented RESTful APIs',
            '• Designed responsive UI',
        ],
    },
    {
        title: 'TechTrend Innovations',
        content: 'Junior Software Engineer',
        date: 'Mar 2021 - Jun 2022',
        image: [
            {
                id: 1,
                src: '/images/techtrend/techtrend1.jpg',
                alt: 'TechTrend project dashboard'
            },
            {
                id: 2,
                src: '/images/techtrend/techtrend2.jpg',
                alt: 'TechTrend team coding session'
            },
        ],
        summaryPoints: [
            'Contributed to microservices •',
            'Automated CI/CD pipelines •',
            'Performed unit testing •',
        ],
    },
];

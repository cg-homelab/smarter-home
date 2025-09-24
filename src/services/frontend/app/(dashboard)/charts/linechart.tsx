'use client';

import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
//import LinePlot from '@/components/ui/line-plot';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

interface ChartData {
    label: string;
    data: number[];
    borderColor: string;
    backgroundColor: string;
}

export function LineChart({
  labels,
  datasets
}: {
  labels: string[];
  datasets: ChartData[];
}) {
    const options = {
        responsive: true,
        // plugins: {
        //     legend: {
        //     position: 'top' as const,
        //     },
        //     title: {
        //     display: true,
        //     text: 'Chart.js Line Chart',
        //     },
        // },
    };


    const data = {
    labels,
    datasets: datasets
    };
  return (
    <Line data={data} options={options} />
  );
}
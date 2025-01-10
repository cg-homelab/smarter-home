import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle
} from '@/components/ui/card';
import { LineChart } from './linechart';
import { faker } from '@faker-js/faker';

export default function ChartsPage() {

  const labels = ['January', 'February', 'March', 'April', 'May', 'June', 'July'];
  const data = [
        {
        label: 'Dataset 1',
        data: labels.map(() => faker.number.int({ min: -1000, max: 1000 })),
        borderColor: 'rgb(255, 99, 132)',
        backgroundColor: 'rgba(255, 99, 132, 0.5)',
        },
        {
        label: 'Dataset 2',
        data: labels.map(() => faker.number.int({ min: -1000, max: 1000 })),
        borderColor: 'rgb(53, 162, 235)',
        backgroundColor: 'rgba(53, 162, 235, 0.5)',
        },
    ];
  const earnings = [
    {month: 'January', amount: 1000},
    {month: 'February', amount: 4000},
    {month: 'March', amount: 3000},
    {month: 'April', amount: 5000},
  ]
  return (
    <Card>
      <CardHeader>
        <CardTitle>Charts</CardTitle>
        <CardDescription>todo</CardDescription>
      </CardHeader>
      <CardContent>
        <LineChart labels={labels} datasets={data}/>
      </CardContent>
    </Card>
  );
}



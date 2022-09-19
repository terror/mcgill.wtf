import { Course } from './course';

export type Payload = {
  time: number;
  courses: [Course];
};

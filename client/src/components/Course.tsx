import React from 'react';
import { Box, Flex, Link, Text } from '@chakra-ui/react';
import { Course as CourseType } from '../lib/course';

interface CourseProps {
  course: CourseType;
}

export const Course: React.ElementType = (props: CourseProps) => {
  return (
    <Box>
      <Text fontWeight='bold'>
        <Link href={props.course.url} isExternal>
          {props.course.subject} {props.course.code}: {props.course.title}
        </Link>
      </Text>
      <Text fontSize='sm'>
        <Link href={props.course.faculty_url} isExternal>
          {props.course.faculty.replace('&amp;', ' & ')}
        </Link>{' '}
        | {props.course.department.replace('&amp;', ' & ')} |{' '}
        {props.course.level} | {props.course.terms.join(', ')}
      </Text>
      <Text fontSize='sm'>{props.course.description}</Text>
      <Text fontSize='sm'>{props.course.instructors}</Text>
    </Box>
  );
};

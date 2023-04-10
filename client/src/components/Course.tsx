import React from 'react';
import _ from 'lodash';
import { Box, Link, Text } from '@chakra-ui/react';
import { Course as CourseType } from '../lib/Course';

interface CourseProps {
  query: string | null;
  course: CourseType;
}

const highlight = (text: string, query: string | null) => {
  if (query === null) return <Text as='span'>{text}</Text>;

  return (
    <Text as='span'>
      {text
        .split(new RegExp(`(${_.escapeRegExp(query)})`, 'gi'))
        .map((part, i) => (
          <Text
            as={part.toLowerCase() === query.toLowerCase() ? 'mark' : 'span'}
            key={i}
          >
            {part}
          </Text>
        ))}
    </Text>
  );
};

export const Course: React.ElementType = (props: CourseProps) => {
  return (
    <Box>
      <Text fontWeight='bold'>
        <Link href={props.course.url} isExternal>
          {highlight(
            `${props.course.subject} ${props.course.code}: ${props.course.title}`,
            props.query
          )}
        </Link>
      </Text>
      <Text fontWeight='medium' fontSize='sm'>
        <Link href={props.course.faculty_url} isExternal>
          {props.course.faculty.replace('&amp;', ' & ')}
        </Link>{' '}
        | {props.course.department.replace('&amp;', ' & ')} |{' '}
        {props.course.level} | {props.course.terms.join(', ')}
      </Text>
      <Text fontSize='sm'>
        {highlight(props.course.description, props.query)}
      </Text>
      <Text fontWeight='medium' fontSize='sm'>
        {props.course.instructors}
      </Text>
    </Box>
  );
};

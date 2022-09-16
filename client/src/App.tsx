import React, { useState } from 'react';

import {
  Alert,
  AlertIcon,
  Box,
  Center,
  Flex,
  Heading,
  Image,
  Input,
  Link,
  Stack,
  Text,
  Wrap,
} from '@chakra-ui/react';

type Payload = {
  time: number;
  courses: [Course];
};

type Course = {
  id: string;
  title: string;
  code: string;
  subject: string;
  level: string;
  url: string;
  department: string;
  department_url: string;
  terms: [string];
  description: string;
  instructors: string;
};

const App: React.ElementType = () => {
  const [payload, setPayload] = useState<Payload | undefined>(undefined);

  const handleChange = async (event: any) => {
    const res = await fetch('/search?query=' + event.target.value);
    console.log(res);
    const json = await res.json();
    console.log(json);
    setPayload(json);
  };

  return (
    <Center padding='1em'>
      <Stack alignItems='center' width='50%'>
        <Wrap>
          <Heading as='h1' size='2xl'>
            mcgill.wtf
          </Heading>
          <Image src='./src/assets/mcgill.png' width='4em' />
        </Wrap>
        <Text>
          A low-latency full-text search of mcgill's entire course catalog
        </Text>
        <Input
          placeholder='Search for a course'
          onChange={(event) => handleChange(event)}
        />
        <Stack alignItems='right'>
          {payload && (
            <Alert status='success'>
              <AlertIcon />
              Found {payload.courses.length} results ({payload.time} ms)
            </Alert>
          )}
          {payload &&
            payload.courses.map((course: Course, _: number) => {
              return (
                <Flex>
                  <Box ml='3'>
                    <Text fontWeight='bold'>
                      <Link href={course.url} isExternal>
                        {course.subject} {course.code}: {course.title}
                      </Link>
                    </Text>
                    <Text fontSize='sm'>
                      {course.department} | {course.level} |{' '}
                      {course.terms.join(', ')}
                    </Text>
                    <Text fontSize='sm'>{course.description}</Text>
                    <Text fontSize='sm'>{course.instructors}</Text>
                  </Box>
                </Flex>
              );
            })}
        </Stack>
      </Stack>
    </Center>
  );
};

export default App;

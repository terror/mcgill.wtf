import React, { useState, useReducer } from 'react';

import {
  Alert,
  AlertIcon,
  Box,
  Center,
  Container,
  Flex,
  Heading,
  Image,
  Input,
  InputGroup,
  InputLeftElement,
  Link,
  Stack,
  Text,
  Wrap,
} from '@chakra-ui/react';

import { SearchIcon } from '@chakra-ui/icons';
import { Course } from './components/Course';
import { Payload } from './lib/payload';
import { Course as CourseType } from './lib/course';
import { search } from './lib/search';

const App: React.ElementType = () => {
  const [value, setValue] = useState<string>('');
  const [payload, setPayload] = useState<Payload | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);

  const handleInputChange = async (value: string) => {
    try {
      setPayload(await search(value));
      setValue(value);
    } catch (error) {
      setError(error instanceof Error ? error.message : 'Unknown Error');
    }
  };

  const handleExampleClick = (index: number) => {
    handleInputChange(examples[index]);
  };

  const examples = ['@subject:comp', '@code:251', '@level:{undergraduate}'];

  return (
    <Center padding='1em'>
      <Stack alignItems='center' width='50%'>
        <Wrap>
          <Heading as='h1' size='2xl'>
            mcgill.wtf
          </Heading>
          <Image src='./mcgill.png' width='4em' />
        </Wrap>
        <Text>
          A low-latency full-text search of mcgill's entire course catalog
        </Text>
        <Text>
          Try out queries like
          {examples.map((example: string, index: number) => (
            <Text
              key={index}
              as='span'
              fontWeight='bold'
              onClick={() => handleExampleClick(index)}
            >
              {' '}
              <Link style={{ textDecoration: 'none' }}>{example}</Link>
            </Text>
          ))}
        </Text>
        <br />
        <InputGroup>
          <InputLeftElement
            pointerEvents='none'
            children={<SearchIcon color='gray.300' />}
          />
          <Input
            value={value}
            placeholder='Search for a course'
            onChange={(event: React.ChangeEvent<HTMLInputElement>) =>
              handleInputChange(event.target.value)
            }
          />
        </InputGroup>
        <Stack alignItems='right' width='100%'>
          {payload && (
            <Alert status='success'>
              <AlertIcon />
              Found {payload.courses.length} results ({payload.time} ms)
            </Alert>
          )}
          {error && (
            <Alert status='error'>
              <AlertIcon />
              error: {error}
            </Alert>
          )}
          {payload &&
            payload.courses.map((course: CourseType, index: number) => {
              return <Course key={index} course={course} />;
            })}
        </Stack>
      </Stack>
    </Center>
  );
};

export default App;

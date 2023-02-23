import React, { useState, useReducer, useCallback, useRef } from 'react';

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
  StackItem,
  Text,
  Wrap,
} from '@chakra-ui/react';

import { Course as CourseType } from './lib/course';
import { Course } from './components/Course';
import { Payload } from './lib/payload';
import { SearchIcon } from '@chakra-ui/icons';
import { search } from './lib/search';

const debounce = (f: (v: string) => void, delay: number) => {
  let lastTimeout: number = 0;
  return (value: string) => {
    if (lastTimeout) clearTimeout(lastTimeout);
    lastTimeout = setTimeout(() => {
      f(value);
    }, delay);
  };
};

const App: React.ElementType = () => {
  const [payload, setPayload] = useState<Payload | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleInputChange = useCallback(
    debounce(async (value: string) => {
      try {
        setPayload(await search(value));
      } catch (error) {
        setError(error instanceof Error ? error.message : 'Unknown Error');
      }
    }, 150),
    []
  );

  const handleExampleClick = (index: number) => {
    if (inputRef.current) inputRef.current.value = examples[index];
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
          <Image src='/assets/mcgill.png' width='3.5rem' />
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
        <InputGroup>
          <InputLeftElement
            pointerEvents='none'
            children={<SearchIcon color='gray.300' />}
          />
          <Input
            placeholder='Search for a course'
            onChange={(event: React.ChangeEvent<HTMLInputElement>) =>
              handleInputChange(event.target.value)
            }
            ref={inputRef}
          />
        </InputGroup>
        <Stack alignItems='right' width='100%'>
          {payload && (
            <Alert status='success' borderRadius='0.5rem'>
              <AlertIcon />
              Found {payload.courses.length} results ({payload.time} ms)
            </Alert>
          )}
          {error && (
            <Alert status='error' borderRadius='0.5rem'>
              <AlertIcon />
              {error}
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

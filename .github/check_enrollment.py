"""Verify a spring assignment's mapping without displaying or uploading credentials."""
import json
import os
from pathlib import Path
import re
from urllib.parse import urlparse


def validate(config, env):
    student = env.get('STUDENT_GITHUB', '')
    if not re.fullmatch(r'[A-Za-z0-9](?:[A-Za-z0-9-]{0,37}[A-Za-z0-9])?', student):
        raise ValueError('Missing or invalid assigned GitHub login.')
    owner, name = env['GITHUB_REPOSITORY'].split('/', 1)
    expected = config['prefix'] + '-' + student
    if owner.lower() != 'learningos' or name.removeprefix('preparing-').lower() != expected.lower():
        raise ValueError('Repository does not match the assigned course student.')
    course_id = env.get('COURSE_ID', '')
    if not course_id.isdecimal() or int(course_id) <= 0:
        raise ValueError('Missing or invalid spring course identifier.')
    if not env.get('COURSE_TOKEN'):
        raise ValueError('The spring course token is unavailable.')
    endpoint = urlparse(env.get('COURSE_API_URL', ''))
    if endpoint.scheme != 'https' or endpoint.hostname != 'api.opencamp.cn':
        raise ValueError('The OpenCamp upload endpoint is unavailable or invalid.')
    return student


def main():
    config = json.loads(Path('.github/spring-course.json').read_text())
    student = validate(config, os.environ)
    text = f"{config['title']}\n\nAssigned student: {student}\n\nSpring course credentials are available. No score was uploaded.\n"
    print('Student mapping and spring course credentials are configured. No score was uploaded.')
    if os.environ.get('GITHUB_STEP_SUMMARY'):
        with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as summary:
            summary.write(text)


if __name__ == '__main__':
    main()

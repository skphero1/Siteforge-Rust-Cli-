#!/usr/bin/env python3
"""Validate this Markdown template using only Python's standard library."""
import argparse
import hashlib
import json
from pathlib import Path
import re
from urllib.parse import unquote, urlsplit

SKILLS = (
    'brainstorming', 'writing-plans', 'test-driven-development',
    'systematic-debugging', 'verification-before-completion',
    'requesting-code-review', 'finishing-a-development-branch',
)
REQUIRED = (
    'AGENTS.md', 'CLAUDE.md', 'README.md', '.gitignore',
    'docs/development-standards.md', 'docs/agent-compatibility.md',
    'docs/new-project-checklist.md', 'docs/technology-profiles/README.md',
    'docs/technology-profiles/wordpress-php-composer.md',
    'docs/technology-profiles/nextjs-react-typescript.md',
    'docs/technology-profiles/python-fastapi-ai-agents.md',
    'docs/technology-profiles/rust-cargo.md',
    '.agents/skills/UPSTREAM.md', '.agents/skills/LICENSE.superpowers',
    '.agents/skills/upstream-lock.json', 'scripts/verify-template.py',
    'tests/test_template_validation.py',
)


def validate(root):
    errors = []
    for name in REQUIRED:
        p = root / name
        if not p.is_file() or not p.stat().st_size:
            errors.append(f'missing required file: {name}')
    for name in SKILLS:
        p = root / '.agents/skills' / name / 'SKILL.md'
        if not p.is_file():
            errors.append(f'missing skill: {name}')
            continue
        text = p.read_text()
        front = re.match(r'\A---\n(.*?)\n---\n', text, re.S)
        if not front:
            errors.append(f'missing skill frontmatter: {name}')
            continue
        fields = dict(re.findall(r'^(name|description):\s*(.+)$', front[1], re.M))
        if fields.get('name') != name:
            errors.append(f'skill name mismatch: {name}')
        if not fields.get('description', '').strip(' \"\''):
            errors.append(f'missing skill description: {name}')
    lockpath = root / '.agents/skills/upstream-lock.json'
    try:
        lock = json.loads(lockpath.read_text())
        if lock.get('repository') != 'https://github.com/obra/superpowers':
            errors.append('unexpected upstream repository')
        if not re.fullmatch(r'[0-9a-f]{40}', lock.get('commit', '')):
            errors.append('invalid upstream commit')
        if lock.get('license') != 'MIT' or lock.get('skills') != list(SKILLS):
            errors.append('invalid upstream selection or license')
        hashes = lock.get('upstream_files', {})
        if not isinstance(hashes, dict) or not hashes:
            errors.append('missing upstream hash manifest')
        else:
            for name in SKILLS:
                if f'.agents/skills/{name}/UPSTREAM.md' not in hashes:
                    errors.append(f'missing upstream snapshot: {name}')
            for relative, expected in hashes.items():
                p = (root / relative).resolve()
                if not p.is_relative_to(root):
                    errors.append(f'upstream path escapes repository: {relative}')
                elif not p.is_file() or hashlib.sha256(p.read_bytes()).hexdigest() != expected:
                    errors.append(f'upstream hash mismatch: {relative}')
        provenance = root / '.agents/skills/UPSTREAM.md'
        if provenance.is_file() and lock.get('commit', '') not in provenance.read_text():
            errors.append('upstream pin missing from provenance')
        license_path = root / '.agents/skills/LICENSE.superpowers'
        if license_path.is_file() and 'Copyright (c) 2025 Jesse Vincent' not in license_path.read_text():
            errors.append('missing upstream copyright notice')
    except (OSError, ValueError, TypeError, AttributeError) as exc:
        errors.append(f'invalid upstream manifest: {exc}')
    # Check maintained Markdown and active skills. Immutable upstream snapshots and
    # supporting assets retain historical links to the full upstream repository.
    markdown = [root / 'README.md', root / 'AGENTS.md', root / 'CLAUDE.md']
    markdown += list((root / 'docs').rglob('*.md'))
    markdown += [root / '.agents/skills/UPSTREAM.md']
    markdown += [root / '.agents/skills' / name / 'SKILL.md' for name in SKILLS]
    for p in markdown:
        if not p.is_file():
            continue
        text = re.sub(r'```.*?```', '', p.read_text(), flags=re.S)
        for target in re.findall(r'\[[^\]\n]*\]\(([^)\s]+)\)', text):
            url = urlsplit(target.strip('<>'))
            if url.scheme or url.netloc or not url.path:
                continue
            destination = (p.parent / unquote(url.path)).resolve()
            if not destination.is_relative_to(root):
                errors.append(f'reference escapes repository: {p.relative_to(root)} -> {target}')
            elif not destination.exists():
                errors.append(f'broken local reference: {p.relative_to(root)} -> {target}')
    return errors


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    errors = validate(args.root.resolve())
    for error in errors:
        print(f'FAIL: {error}')
    if errors:
        return 1
    print(f'PASS: required files, {len(SKILLS)} skills, metadata, local file references, upstream pin and asset hashes')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())

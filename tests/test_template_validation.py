"""Exercise the validator against damaged disposable copies of the template."""
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class TemplateValidationTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / 'template'
        shutil.copytree(ROOT, self.root, ignore=shutil.ignore_patterns('.git', '__pycache__'))

    def run_validator(self):
        return subprocess.run(
            [sys.executable, str(ROOT / 'scripts/verify-template.py'), '--root', str(self.root)],
            capture_output=True, text=True, check=False,
        )

    def assert_invalid(self, expected):
        result = self.run_validator()
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn(expected, result.stdout)

    def test_valid_template(self):
        result = self.run_validator()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_missing_required_file(self):
        (self.root / 'AGENTS.md').unlink()
        self.assert_invalid('missing required file: AGENTS.md')

    def test_incorrect_skill_metadata(self):
        p = self.root / '.agents/skills/brainstorming/SKILL.md'
        p.write_text(p.read_text().replace('name: brainstorming', 'name: wrong'))
        self.assert_invalid('skill name mismatch')

    def test_missing_description(self):
        p = self.root / '.agents/skills/brainstorming/SKILL.md'
        p.write_text(p.read_text().replace('description:', 'other:'))
        self.assert_invalid('missing skill description')

    def test_broken_local_reference(self):
        with (self.root / 'README.md').open('a') as f:
            f.write('\n[missing](docs/nonexistent.md)\n')
        self.assert_invalid('broken local reference')

    def test_altered_upstream_asset(self):
        p = self.root / '.agents/skills/test-driven-development/writing-good-tests.md'
        p.write_text(p.read_text() + '\nchanged\n')
        self.assert_invalid('upstream hash mismatch')

    def test_invalid_pin(self):
        p = self.root / '.agents/skills/upstream-lock.json'
        p.write_text(p.read_text().replace('2ce8aa15d05bd57213e6f746f953aafb7ded7a8b', 'main'))
        self.assert_invalid('invalid upstream commit')

    def test_reference_outside_repository(self):
        with (self.root / 'README.md').open('a') as f:
            f.write('\n[outside](../outside.md)\n')
        self.assert_invalid('reference escapes repository')


if __name__ == '__main__':
    unittest.main()

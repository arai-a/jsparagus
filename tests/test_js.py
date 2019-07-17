""" Tests for the JS parser. """

import unittest
import jsparagus.lexer
from js_parser.parser import parse_Script


class ESTestCase(unittest.TestCase):
    def assert_parses(self, s):
        parse_Script(s)

    def assert_incomplete(self, s):
        """Assert that s fails to parse with UnexpectedEndError.

        (This should be the case if `s` is a prefix of a valid Script.)
        """
        self.assertRaises(jsparagus.lexer.UnexpectedEndError,
                          lambda: parse_Script(s))

    def assert_syntax_error(self, s):
        """Assert that s fails to parse."""
        self.assertRaises(jsparagus.lexer.SyntaxError,
                          lambda: parse_Script(s))

    # === Tests!

    def test_asi_at_end(self):
        self.assert_parses("3 + 4")
        self.assert_syntax_error("3 4")
        self.assert_incomplete("3 +")
        self.assert_incomplete("{")
        self.assert_incomplete("{;")

    def test_if_else(self):
        self.assert_parses("if (x) f();")
        self.assert_incomplete("if (x)")
        self.assert_parses("if (x) f(); else g();")
        self.assert_incomplete("if (x) f(); else")
        self.assert_parses("if (x) if (y) g(); else h();")
        self.assert_parses("if (x) if (y) g(); else h(); else j();")

    def test_lexer_decimal(self):
        self.assert_parses("0.")
        self.assert_parses(".5")
        self.assert_syntax_error(".")

    def test_arrow(self):
        self.assert_parses("x => x")
        self.assert_parses("f = x => x;")
        self.assert_parses("(x, y) => [y, x]")
        self.assert_parses("f = (x, y) => {}")
        self.assert_syntax_error("(x, y) => {x: x, y: y}")

    def test_invalid_character(self):
        self.assert_syntax_error("\0")
        self.assert_syntax_error("—x;")
        self.assert_syntax_error("const ONE_THIRD = 1 ÷ 3;")


if __name__ == '__main__':
    unittest.main()
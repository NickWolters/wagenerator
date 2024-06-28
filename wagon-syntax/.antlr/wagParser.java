// Generated from /home/technique/dev/wagon2/wagon-syntax/wag.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast", "CheckReturnValue"})
public class wagParser extends Parser {
	static { RuntimeMetaData.checkVersion("4.13.1", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		Metadata=1, MetaDelim=2, Meta=3, Include=4, Config=5, Rule=6, Rhs=7, Weight=8, 
		Chunk=9, EbnfType=10, ChunkP=11, Symbol=12, NonTerminal=13, NTArgs=14, 
		AttrIdentifierList=15, Terminal=16, Assignment=17, Expression=18, SubProc=19, 
		If=20, Disjunct=21, Conjunct=22, Inverse=23, Comparison=24, CompOp=25, 
		Sum=26, SumP=27, SumOp=28, Term=29, TermP=30, TermOp=31, Factor=32, Atom=33, 
		Identifier=34, AttrIdentifier=35, AttrSpec=36, Dictionary=37, Bool=38, 
		Num=39, Float=40, String=41;
	public static final int
		RULE_wag = 0;
	private static String[] makeRuleNames() {
		return new String[] {
			"wag"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "Metadata", "MetaDelim", "Meta", "Include", "Config", "Rule", "Rhs", 
			"Weight", "Chunk", "EbnfType", "ChunkP", "Symbol", "NonTerminal", "NTArgs", 
			"AttrIdentifierList", "Terminal", "Assignment", "Expression", "SubProc", 
			"If", "Disjunct", "Conjunct", "Inverse", "Comparison", "CompOp", "Sum", 
			"SumP", "SumOp", "Term", "TermP", "TermOp", "Factor", "Atom", "Identifier", 
			"AttrIdentifier", "AttrSpec", "Dictionary", "Bool", "Num", "Float", "String"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "wag.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public wagParser(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@SuppressWarnings("CheckReturnValue")
	public static class WagContext extends ParserRuleContext {
		public TerminalNode EOF() { return getToken(wagParser.EOF, 0); }
		public TerminalNode Metadata() { return getToken(wagParser.Metadata, 0); }
		public List<TerminalNode> Rule() { return getTokens(wagParser.Rule); }
		public TerminalNode Rule(int i) {
			return getToken(wagParser.Rule, i);
		}
		public WagContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_wag; }
	}

	public final WagContext wag() throws RecognitionException {
		WagContext _localctx = new WagContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_wag);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(3);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==Metadata) {
				{
				setState(2);
				match(Metadata);
				}
			}

			setState(8);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==Rule) {
				{
				{
				setState(5);
				match(Rule);
				}
				}
				setState(10);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(11);
			match(EOF);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static final String _serializedATN =
		"\u0004\u0001)\u000e\u0002\u0000\u0007\u0000\u0001\u0000\u0003\u0000\u0004"+
		"\b\u0000\u0001\u0000\u0005\u0000\u0007\b\u0000\n\u0000\f\u0000\n\t\u0000"+
		"\u0001\u0000\u0001\u0000\u0001\u0000\u0000\u0000\u0001\u0000\u0000\u0000"+
		"\u000e\u0000\u0003\u0001\u0000\u0000\u0000\u0002\u0004\u0005\u0001\u0000"+
		"\u0000\u0003\u0002\u0001\u0000\u0000\u0000\u0003\u0004\u0001\u0000\u0000"+
		"\u0000\u0004\b\u0001\u0000\u0000\u0000\u0005\u0007\u0005\u0006\u0000\u0000"+
		"\u0006\u0005\u0001\u0000\u0000\u0000\u0007\n\u0001\u0000\u0000\u0000\b"+
		"\u0006\u0001\u0000\u0000\u0000\b\t\u0001\u0000\u0000\u0000\t\u000b\u0001"+
		"\u0000\u0000\u0000\n\b\u0001\u0000\u0000\u0000\u000b\f\u0005\u0000\u0000"+
		"\u0001\f\u0001\u0001\u0000\u0000\u0000\u0002\u0003\b";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
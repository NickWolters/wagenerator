// Generated from /home/technique/dev/wagon2/wagon-syntax/wag.g4 by ANTLR 4.13.1
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by
 * {@link wagParser}.
 */
public interface wagListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by {@link wagParser#wag}.
	 * @param ctx the parse tree
	 */
	void enterWag(wagParser.WagContext ctx);
	/**
	 * Exit a parse tree produced by {@link wagParser#wag}.
	 * @param ctx the parse tree
	 */
	void exitWag(wagParser.WagContext ctx);
}
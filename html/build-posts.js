const fs = require("fs");
const path = require("path");

function parseFrontmatter(content) {
	const match = content.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/);
	if (!match) return null;

	const frontmatterBlock = match[1];
	const markdown = match[2];

	const frontmatter = {};
	frontmatterBlock.split("\n").forEach((line) => {
		const [key, ...valueParts] = line.split(":");
		if (key && valueParts.length) {
			frontmatter[key.trim()] = valueParts.join(":").trim();
		}
	});

	return { frontmatter, markdown };
}

const postsDir = path.join(__dirname, "posts");
const posts = fs
	.readdirSync(postsDir)
	.filter((file) => file.endsWith(".md"))
	.map((file) => {
		const content = fs.readFileSync(path.join(postsDir, file), "utf-8");
		const parsed = parseFrontmatter(content);

		if (!parsed) {
			console.warn(`Warning: ${file} has invalid frontmatter`);
			return null;
		}

		return {
			slug: file.replace(".md", ""),
			...parsed.frontmatter,
			content: parsed.markdown,
		};
	})
	.filter(Boolean)
	.sort((a, b) => new Date(b.date) - new Date(a.date));

fs.writeFileSync(
	path.join(__dirname, "posts.json"),
	JSON.stringify(posts, null, 2),
);

console.log(`Generated posts.json with ${posts.length} posts`);

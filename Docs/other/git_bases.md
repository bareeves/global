You can combine the **`git add`**, **`git commit`**, and **`git push`** commands into a single line using the `&&` operator or a semicolon (`;`). Here's how you can do it:

### Using `&&` (Recommended)
The `&&` ensures that each command executes only if the previous one succeeds:

```bash
git add . && git commit -m "Your commit message" && git push
```

### Using `;`
The `;` executes all commands regardless of whether the previous one succeeded:

```bash
git add .; git commit -m "Your commit message"; git push
```

### Explanation:
- **`git add .`**: Stages all changes in your working directory.
- **`git commit -m "Your commit message"`**: Commits the staged changes with a message.
- **`git push`**: Pushes the committed changes to the remote repository.

### A One-Liner for Frequent Updates
If you're making frequent updates and want to skip the commit message prompt, you can write a shell alias or function. However, avoid skipping commit messages entirely for good version control practices.
# A Comprehensive Guide to Setting Up Git and SSH for Smooth Workflow

### Basic Git Commands
Git is an essential tool for version control and collaboration. Here are a few common commands:

1. **Stage Changes:**
   ```bash
   git add .
   ```
   Stages all the changes in your working directory.

2. **Commit Changes:**
   ```bash
   git commit -m 'test02'
   ```
   Saves the staged changes with a message (`test02` in this example).

3. **Push Changes to Remote Repository:**
   ```bash
   git push
   ```
   Sends your local commits to the remote repository.

---

### Cloning a Repository
To get a copy of an existing repository:

1. **Clone via SSH:**
   ```bash
   git clone git@github.com:evadawnley/global.git
   ```
   This command creates a local copy of the repository.

2. **Push Changes to the Main Branch:**
   ```bash
   git push origin main
   ```

---

### Set Up SSH Authentication for GitHub

Using SSH authentication eliminates the need to repeatedly enter your username and password or personal access token.

#### 1. Generate an SSH Key
If you don’t already have an SSH key, create one:

```bash
ssh-keygen -t ed25519 -C "your-email@example.com"
```

- Press **Enter** to save the key to the default location (`~/.ssh/id_ed25519`).
- Optionally, add a passphrase for enhanced security.

#### 2. Add the SSH Key to GitHub
To link your SSH key to your GitHub account:

1. Copy the public key:
   ```bash
   cat ~/.ssh/id_ed25519.pub
   ```
2. Go to your GitHub account and navigate to:
   **Settings** > **SSH and GPG keys** > **New SSH key**.
3. Paste the copied public key into the provided field and click **Add SSH Key**.

#### 3. Clone the Repository Using SSH
Now, use the SSH URL instead of HTTPS when cloning repositories:

```bash
git clone git@github.com:your-username/your-repo-name.git
```

#### 4. Push Without Prompting
With SSH configured, you can push changes without needing to enter credentials every time:

```bash
git push origin main
```

---

By following this guide, you’ll streamline your Git workflow and make your development process more efficient.

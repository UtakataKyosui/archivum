gh auth login 
gh auth setup-git
gh extension install UtakataKyosui/gh-wheel

echo 'eval "$(atuin init bash)"' >> ~/.bashrc
echo 'eval "$(gh wheel completion bash)"' >> ~/.bashrc
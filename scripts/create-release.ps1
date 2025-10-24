# PowerShell script to create a new release for the DECENTRALIZED-APP project

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [Parameter(Mandatory=$false)]
    [string]$CommitMessage = "Release $Version"
)

# Check if tag already exists
try {
    $tagExists = git rev-parse "$Version" 2>$null
    if ($tagExists) {
        Write-Error "Tag $Version already exists!"
        exit 1
    }
} catch {
    # Tag doesn't exist, which is what we want
}

Write-Host "Creating release $Version..."

# Update version in root Cargo.toml if it exists and differs
$rootCargo = "Cargo.toml"
if (Test-Path $rootCargo) {
    $content = Get-Content $rootCargo
    $currentVersionLine = $content | Where-Object { $_ -match "^version = " }
    if ($currentVersionLine) {
        $currentVersion = $currentVersionLine -split '"' | Select-Object -Index 1
        if ($currentVersion -ne $Version.TrimStart('v')) {
            Write-Host "Updating version in $rootCargo from $currentVersion to $($Version.TrimStart('v'))"
            $content = $content -replace "^version = `".*`"", "version = `"$($Version.TrimStart('v'))`""
            Set-Content $rootCargo $content
        }
    }
}

# Update versions in all crate Cargo.toml files
Get-ChildItem -Recurse -Filter "Cargo.toml" | Where-Object { $_.FullName -notlike "*target*" } | ForEach-Object {
    $content = Get-Content $_.FullName
    $currentVersionLine = $content | Where-Object { $_ -match "^version = " }
    if ($currentVersionLine) {
        $currentVersion = $currentVersionLine -split '"' | Select-Object -Index 1
        if ($currentVersion -ne $Version.TrimStart('v')) {
            Write-Host "Updating version in $($_.FullName) from $currentVersion to $($Version.TrimStart('v'))"
            $content = $content -replace "^version = `".*`"", "version = `"$($Version.TrimStart('v'))`""
            Set-Content $_.FullName $content
        }
    }
}

# Commit version changes if any
if (-not (git diff --quiet)) {
    Write-Host "Committing version updates..."
    git add .
    git commit -m "Bump version to $($Version.TrimStart('v'))"
}

# Create and push tag
Write-Host "Creating and pushing tag $Version..."
git tag -a "$Version" -m "$CommitMessage"
git push origin "$Version"

Write-Host "Release $Version created and pushed!"
Write-Host "GitHub Actions will automatically create the release and publish packages."
Write-Host "View progress at: https://github.com/attakdefand/DECENTRALIZED-APP/actions"
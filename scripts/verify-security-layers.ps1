# Script to verify all security layers are implemented according to the protection matrix

Write-Host "Verifying Security Layers Implementation" -ForegroundColor Green

# Define the security layers from the matrix
$SecurityLayers = @{
    "Layer1" = @{
        "name" = "Governance & Policy"
        "components" = @("Policy Catalog", "Exception Management", "Audit & Assurance")
        "evidence_files" = @("docs/security/POLICY-CATALOG.md", "docs/security/EXCEPTIONS.md")
    }
    "Layer2" = @{
        "name" = "Identity & Access Control"
        "components" = @("AuthN", "AuthZ", "Session & Token Hygiene", "Secrets Hygiene")
        "evidence_files" = @("docs/security/IAM-RBAC-MAP.md")
    }
    "Layer3" = @{
        "name" = "Application Security"
        "components" = @("Input Protection", "Output Protection", "Business Logic Controls", "Dependency Safety", "Runtime Protections")
        "evidence_files" = @()
    }
    "Layer4" = @{
        "name" = "API & Gateway Security"
        "components" = @("Protocol Safety", "Abuse Mitigation", "Auth at Edge", "Data Filtering", "Allowlisting")
        "evidence_files" = @()
    }
    "Layer5" = @{
        "name" = "Data Security"
        "components" = @("Data Classification", "Data-in-Transit", "Data-at-Rest", "Data Minimization", "Backup & Restore")
        "evidence_files" = @("docs/data/PRIVACY-DATA-MAP.md")
    }
    "Layer6" = @{
        "name" = "Network & Infrastructure Security"
        "components" = @("Perimeter Defense", "Segmentation", "OSI Hardening", "Host Hardening", "Secrets on Host")
        "evidence_files" = @("infra/k8s/helm/templates/network-policy.yaml", "infra/k8s/helm/templates/pod-security.yaml")
    }
    "Layer7" = @{
        "name" = "Resilience & Availability"
        "components" = @("Redundancy", "Traffic Protection", "Graceful Degradation", "Disaster Recovery")
        "evidence_files" = @("infra/k8s/helm/templates/pod-disruption-budget.yaml")
    }
    "Layer8" = @{
        "name" = "Observability & Detection"
        "components" = @("Telemetry", "Security Detection", "Forensics & Evidence", "Incident Response")
        "evidence_files" = @("docs/observability/OTEL-SETUP.md")
    }
    "Layer9" = @{
        "name" = "Software Supply Chain"
        "components" = @("Artifact Integrity", "Dependency Trust", "CI/CD Gatekeeping", "Runtime Drift Control")
        "evidence_files" = @("docs/ci/SUPPLY-CHAIN.md", ".github/workflows/supply-chain-security.yml")
    }
}

# Verification results
$TotalLayers = $SecurityLayers.Count
$ImplementedLayers = 0
$VerificationResults = @()

foreach ($layerKey in $SecurityLayers.Keys) {
    $layerInfo = $SecurityLayers[$layerKey]
    $layerName = $layerInfo.name
    $evidenceFiles = $layerInfo.evidence_files
    
    Write-Host "`nChecking $layerKey - $layerName" -ForegroundColor Yellow
    
    $layerImplemented = $true
    
    # Check for evidence files
    foreach ($file in $evidenceFiles) {
        if (Test-Path $file) {
            Write-Host "  $(echo [224;32m) Found evidence file: $file $(echo [0m)" -ForegroundColor Green
        } else {
            Write-Host "  $(echo [224;31m) Missing evidence file: $file $(echo [0m)" -ForegroundColor Red
            $layerImplemented = $false
        }
    }
    
    # Additional checks based on layer
    if ($layerKey -eq "Layer1") {
        # Check for policy documents
        $policyFiles = @("docs/security/POLICY-CATALOG.md", "docs/security/EXCEPTIONS.md")
        foreach ($policyFile in $policyFiles) {
            if (Test-Path $policyFile) {
                Write-Host "  $(echo [224;32m) Found policy document: $policyFile $(echo [0m)" -ForegroundColor Green
            } else {
                Write-Host "  $(echo [224;31m) Missing policy document: $policyFile $(echo [0m)" -ForegroundColor Red
                $layerImplemented = $false
            }
        }
    }
    
    if ($layerKey -eq "Layer2") {
        # Check for IAM configuration
        if (Test-Path "docs/security/IAM-RBAC-MAP.md") {
            Write-Host "  $(echo [224;32m) Found IAM/RBAC configuration $(echo [0m)" -ForegroundColor Green
        } else {
            Write-Host "  $(echo [224;31m) Missing IAM/RBAC configuration $(echo [0m)" -ForegroundColor Red
            $layerImplemented = $false
        }
    }
    
    if ($layerKey -eq "Layer3") {
        # Check for input validation in services
        $validationFound = $false
        Get-ChildItem -Path "services" -Recurse -Filter "*.rs" | ForEach-Object {
            $content = Get-Content $_.FullName -Raw
            if ($content -match "validate|Validation|sanitiz") {
                $validationFound = $true
                Write-Host "  $(echo [224;32m) Found validation logic in $($_.Name) $(echo [0m)" -ForegroundColor Green
            }
        }
        if (-not $validationFound) {
            Write-Host "  $(echo [224;33m) No explicit validation logic found in services $(echo [0m)" -ForegroundColor Yellow
        }
    }
    
    if ($layerKey -eq "Layer4") {
        # Check for rate limiting
        $rateLimitFound = $false
        Get-ChildItem -Path "services" -Recurse -Filter "*.rs" | ForEach-Object {
            $content = Get-Content $_.FullName -Raw
            if ($content -match "rate.*limit|throttle|quota") {
                $rateLimitFound = $true
                Write-Host "  $(echo [224;32m) Found rate limiting logic in $($_.Name) $(echo [0m)" -ForegroundColor Green
            }
        }
        if (-not $rateLimitFound) {
            Write-Host "  $(echo [224;33m) No explicit rate limiting logic found in services $(echo [0m)" -ForegroundColor Yellow
        }
    }
    
    if ($layerKey -eq "Layer5") {
        # Check for encryption
        $encryptionFound = $false
        Get-ChildItem -Path "crates" -Recurse -Filter "*.rs" | ForEach-Object {
            $content = Get-Content $_.FullName -Raw
            if ($content -match "encrypt|cipher|aes|rsa") {
                $encryptionFound = $true
                Write-Host "  $(echo [224;32m) Found encryption logic in $($_.Name) $(echo [0m)" -ForegroundColor Green
            }
        }
        if (-not $encryptionFound) {
            Write-Host "  $(echo [224;33m) No explicit encryption logic found in crates $(echo [0m)" -ForegroundColor Yellow
        }
    }
    
    if ($layerKey -eq "Layer6") {
        # Check for Kubernetes security configurations
        $k8sSecurityFiles = @(
            "infra/k8s/helm/templates/network-policy.yaml",
            "infra/k8s/helm/templates/pod-security.yaml",
            "infra/k8s/helm/templates/rbac.yaml"
        )
        foreach ($k8sFile in $k8sSecurityFiles) {
            if (Test-Path $k8sFile) {
                Write-Host "  $(echo [224;32m) Found Kubernetes security file: $k8sFile $(echo [0m)" -ForegroundColor Green
            } else {
                Write-Host "  $(echo [224;31m) Missing Kubernetes security file: $k8sFile $(echo [0m)" -ForegroundColor Red
                $layerImplemented = $false
            }
        }
    }
    
    if ($layerKey -eq "Layer7") {
        # Check for resilience mechanisms
        if (Test-Path "infra/k8s/helm/templates/pod-disruption-budget.yaml") {
            Write-Host "  $(echo [224;32m) Found Pod Disruption Budget for resilience $(echo [0m)" -ForegroundColor Green
        } else {
            Write-Host "  $(echo [224;31m) Missing Pod Disruption Budget for resilience $(echo [0m)" -ForegroundColor Red
            $layerImplemented = $false
        }
    }
    
    if ($layerKey -eq "Layer8") {
        # Check for observability
        $observabilityFound = $false
        Get-ChildItem -Path "services" -Recurse -Filter "*.rs" | ForEach-Object {
            $content = Get-Content $_.FullName -Raw
            if ($content -match "tracing|opentelemetry|metrics") {
                $observabilityFound = $true
                Write-Host "  $(echo [224;32m) Found observability logic in $($_.Name) $(echo [0m)" -ForegroundColor Green
            }
        }
        if (-not $observabilityFound) {
            Write-Host "  $(echo [224;33m) No explicit observability logic found in services $(echo [0m)" -ForegroundColor Yellow
        }
    }
    
    if ($layerKey -eq "Layer9") {
        # Check for supply chain security
        $supplyChainFiles = @(
            "docs/ci/SUPPLY-CHAIN.md",
            ".github/workflows/supply-chain-security.yml",
            ".github/workflows/container-security-scan.yml"
        )
        foreach ($supplyChainFile in $supplyChainFiles) {
            if (Test-Path $supplyChainFile) {
                Write-Host "  $(echo [224;32m) Found supply chain security file: $supplyChainFile $(echo [0m)" -ForegroundColor Green
            } else {
                Write-Host "  $(echo [224;31m) Missing supply chain security file: $supplyChainFile $(echo [0m)" -ForegroundColor Red
                $layerImplemented = $false
            }
        }
    }
    
    $VerificationResults += [PSCustomObject]@{
        Layer = $layerKey
        Name = $layerName
        Implemented = $layerImplemented
    }
    
    if ($layerImplemented) {
        $ImplementedLayers++
    }
}

# Summary
Write-Host "`n`n=== Security Layers Verification Summary ===" -ForegroundColor Cyan
Write-Host "Total Layers: $TotalLayers" -ForegroundColor White
Write-Host "Implemented Layers: $ImplementedLayers" -ForegroundColor White
$implementationRate = if ($TotalLayers -gt 0) { [math]::Round(($ImplementedLayers/$TotalLayers)*100, 2) } else { 0 }
Write-Host "Implementation Rate: $implementationRate%" -ForegroundColor White

Write-Host "`nDetailed Results:" -ForegroundColor Cyan
$VerificationResults | Format-Table -AutoSize

# Check Docker security practices
Write-Host "`n=== Docker Security Verification ===" -ForegroundColor Cyan
$Dockerfiles = Get-ChildItem -Path "." -Recurse -Filter "Dockerfile" | Where-Object { $_.FullName -notlike "*target*" }

$DockerSecurityChecks = @{
    "NonRootUser" = 0
    "MinimalBaseImage" = 0
    "ReadOnlyFilesystem" = 0
}

$totalDockerfiles = $Dockerfiles.Count

foreach ($dockerfile in $Dockerfiles) {
    $content = Get-Content $dockerfile.FullName -Raw
    
    # Check for non-root user
    if ($content -match "USER.*[1-9]") {
        $DockerSecurityChecks["NonRootUser"]++
    }
    
    # Check for minimal base images
    if ($content -match "distroless|scratch|alpine") {
        $DockerSecurityChecks["MinimalBaseImage"]++
    }
    
    # Check for read-only filesystem
    if ($content -match "readOnlyRootFilesystem.*true|read_only.*true") {
        $DockerSecurityChecks["ReadOnlyFilesystem"]++
    }
}

Write-Host "Total Dockerfiles: $totalDockerfiles" -ForegroundColor White
foreach ($checkKey in $DockerSecurityChecks.Keys) {
    $count = $DockerSecurityChecks[$checkKey]
    $percentage = if ($totalDockerfiles -gt 0) { [math]::Round(($count/$totalDockerfiles)*100, 2) } else { 0 }
    $checkName = switch ($checkKey) {
        "NonRootUser" { "Non-root user" }
        "MinimalBaseImage" { "Minimal base image" }
        "ReadOnlyFilesystem" { "Read-only filesystem" }
        default { $checkKey }
    }
    Write-Host "$checkName: $count/$totalDockerfiles ($percentage%)" -ForegroundColor White
}

# Check Kubernetes security practices
Write-Host "`n=== Kubernetes Security Verification ===" -ForegroundColor Cyan
$K8sSecurityFiles = @(
    "infra/k8s/helm/templates/network-policy.yaml",
    "infra/k8s/helm/templates/pod-security.yaml",
    "infra/k8s/helm/templates/rbac.yaml",
    "infra/k8s/helm/templates/pod-disruption-budget.yaml"
)

$ImplementedK8sFiles = 0
foreach ($k8sFile in $K8sSecurityFiles) {
    if (Test-Path $k8sFile) {
        $ImplementedK8sFiles++
        Write-Host "$(echo [224;32m) Found $k8sFile $(echo [0m)" -ForegroundColor Green
    } else {
        Write-Host "$(echo [224;31m) Missing $k8sFile $(echo [0m)" -ForegroundColor Red
    }
}

Write-Host "`nKubernetes Security Files: $ImplementedK8sFiles/$($K8sSecurityFiles.Count)" -ForegroundColor White

Write-Host "`nVerification completed." -ForegroundColor Green
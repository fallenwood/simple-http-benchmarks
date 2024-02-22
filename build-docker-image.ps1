Import-Module "./images.psm1"

$container = Get-Container

$images = Get-Images

foreach ($image in $images) {
    $tag = Get-TagName $image.Tag
    $tagBase = Get-ImageTag $image.Tag

    $exist = Invoke-Expression "$container images | Select-String '$tagBase'" | Out-String

    if ([System.String]::IsNullOrWhiteSpace($exist)) {

        Write-Host "Building image for $tag"
    
        Invoke-Expression "$container build -t $tag --file $($image.Dockerfile) $($image.Directory)"
    } else {
        Write-Host "Skipping existing $tag"
    }
}

Write-Host "Docker Images built, run ""pwsh ./benchmark.ps1"" to start benchmarking..."

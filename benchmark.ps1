Import-Module "./images.psm1"

$container = Get-Container
$wrk = Get-Wrk

$images = Get-Images

$port = Get-Port

$concurrences = 1, 100, 125
$duration = 10

$resp = New-Object System.Collections.Generic.List[System.Object]

foreach ($image in $images) {
    $tag = Get-TagName $image.Tag 
    $sha = Invoke-Expression "$container run --rm --network host -d $tag" | Out-String

    Write-Host "Benchmarking $tag with $sha"

    Start-Sleep -Milliseconds 1000

    foreach ($concurrence in $concurrences) {
        Invoke-Expression "$wrk -c $concurrence -d $duration http://127.0.0.1:$port"

        # $resp.Add(@{
        #     Tag = $image.Tag;
        #     NumTasks = $numTask
        #     MaximumSize = $maximumSize
        # })
    }

    Invoke-Expression "$container stop $sha"
}

Write-Host "Benchmarking complete, results:"

# foreach ($res in $resp) {
#     Write-Host "$($res.Tag)`t$($res.NumTasks)`t$($mb)"
# }
